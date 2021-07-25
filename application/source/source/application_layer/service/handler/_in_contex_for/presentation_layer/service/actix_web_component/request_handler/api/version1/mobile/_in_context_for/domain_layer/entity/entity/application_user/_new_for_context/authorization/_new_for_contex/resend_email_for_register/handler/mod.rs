use crate::domain_layer::entity::entity::application_user_registration_confirmation_token::application_user_registration_confirmation_token::ApplicationUserRegistrationConfirmationToken;
use crate::domain_layer::entity::entity::application_user::_core::email::Email;
use crate::domain_layer::error::entity_error::_core::_in_context_for::domain_layer::entity::entity::pre_confirmed_application_user::_new_for_context::pre_confirmed_application_user_error::PreConfirmedApplicationUserError;
use crate::domain_layer::error::entity_error::entity_error::EntityError;
use crate::domain_layer::service::_in_context_for::domain_layer::entity::entity::application_user::_new_for_context::email_sender_trait::EmailSenderTrait;
use crate::infrastructure_layer::error::base_error::base_error::BaseError;
use crate::infrastructure_layer::repository::_in_context_for::entity::entity::application_user_registration_confirmation_token::_new_for_context::_in_context_for::_resource::redis::_new_for_context::base_repository::BaseRepository as ApplicationUserRegistrationConfirmationTokenBaseRepository;
use crate::infrastructure_layer::repository::_in_context_for::entity::entity::pre_confirmed_application_user::_new_for_context::_in_context_for::_resource::postgresql::_new_for_context::base_repository::BaseRepository as PreConfirmedApplicationUserBaseRepository;
use crate::infrastructure_layer::service::_in_context_for::_resource::_new_for_context::aggregate_connection_pool::AggregateConnectionPool;
use crate::infrastructure_layer::service::_in_context_for::_resource::_new_for_context::connection_extractor::ConnectionExtractor;
use crate::infrastructure_layer::service::_in_context_for::domain_layer::entity::entity::application_user::_new_for_context::email_sender::EmailSender;
use crate::presentation_layer::data_transfer_object::request::_in_context_for::presentation_layer::service::actix_web_component::request_handler::api::version1::mobile::_in_context_for::domain_layer::entity::entity::application_user::_new_for_context::authorization::_new_for_context::resend_email_for_register::request::Request;
use redis::Connection;
use std::sync::Arc;

pub struct Handler;

impl Handler {
    pub fn handle(aggregate_connection_pool: Arc<AggregateConnectionPool>, request: Request) -> Result<(), BaseError> { // TODO сделать На Редисе механизм для невозможности почстоянно отравки емэйла. (Сохранять, если отправлено, и проверять, что отпрпавили. удалять по времени)
        if let Some(pre_confirmed_application_user) = PreConfirmedApplicationUserBaseRepository::get_by_application_user_email(
            &*ConnectionExtractor::get_postgresql_connection(&aggregate_connection_pool)?, &Email::new(request.get_application_user_email())
        )? 
        {
            let application_user_registration_confirmation_token: ApplicationUserRegistrationConfirmationToken<'_>;

            let connection: &'_ mut Connection = &mut *ConnectionExtractor::get_redis_connection(&aggregate_connection_pool)?;

            match ApplicationUserRegistrationConfirmationTokenBaseRepository::get_by_pre_confirmed_application_user_id(connection, pre_confirmed_application_user.get_id()?)? {
                Some(existing_application_user_registration_confirmation_token) => {
                    application_user_registration_confirmation_token = existing_application_user_registration_confirmation_token;

                    ApplicationUserRegistrationConfirmationTokenBaseRepository::update_expiration_time(connection, &application_user_registration_confirmation_token)?;
                },
                None => {
                    application_user_registration_confirmation_token = ApplicationUserRegistrationConfirmationToken::new(&pre_confirmed_application_user)?;

                    ApplicationUserRegistrationConfirmationTokenBaseRepository::create(connection, &application_user_registration_confirmation_token)?;
                }
            }
            
            <EmailSender as EmailSenderTrait>::send_application_user_registration_confirmation_token(&application_user_registration_confirmation_token)?;

            return Ok(());
        }

        return Err(BaseError::EntityError(EntityError::PreConfirmedApplicationUserError(PreConfirmedApplicationUserError::NotFound)));
    }
}