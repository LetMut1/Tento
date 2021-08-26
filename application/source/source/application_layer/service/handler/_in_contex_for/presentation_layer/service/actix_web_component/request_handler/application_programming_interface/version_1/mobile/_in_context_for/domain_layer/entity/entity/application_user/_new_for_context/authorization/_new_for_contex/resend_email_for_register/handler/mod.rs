use crate::domain_layer::entity::entity::application_user_registration_confirmation_token::application_user_registration_confirmation_token::ApplicationUserRegistrationConfirmationToken;
use crate::domain_layer::entity::entity::application_user::_component::email::Email;
use crate::domain_layer::error::entity_error::_component::_in_context_for::domain_layer::entity::entity::application_user_pre_confirmed::_new_for_context::application_user_pre_confirmed_error::ApplicationUserPreConfirmedError;
use crate::domain_layer::error::entity_error::entity_error::EntityError;
use crate::domain_layer::repository::_in_context_for::domain_layer::entity::entity::application_user_pre_confirmed::_new_for_context::_in_context_for::_resource::postgresql::_new_for_context::base_repository_trait::BaseRepositoryTrait as ApplicationUserPreConfirmedBaseRepositoryTrait;
use crate::domain_layer::repository::_in_context_for::domain_layer::entity::entity::application_user_registration_confirmation_token::_new_for_context::_in_context_for::_resource::redis::_new_for_context::base_repository_trait::BaseRepositoryTrait as ApplicationUserRegistrationConfirmationTokenBaseRepositoryTrait;
use crate::domain_layer::service::_in_context_for::domain_layer::entity::entity::application_user::_new_for_context::email_sender_trait::EmailSenderTrait;
use crate::domain_layer::service::factory::_in_context_for::domain_layer::entity::entity::application_user_registration_confirmation_token::_new_for_context::factory::Factory as ApplicationUserRegistrationConfirmationTokenFactory;
use crate::infrastructure_layer::error::base_error::base_error::BaseError;
use crate::infrastructure_layer::repository::_in_context_for::domain_layer::entity::entity::application_user_pre_confirmed::_new_for_context::_in_context_for::_resource::postgresql::_new_for_context::base_repository::BaseRepository as ApplicationUserPreConfirmedBaseRepository;
use crate::infrastructure_layer::repository::_in_context_for::domain_layer::entity::entity::application_user_registration_confirmation_token::_new_for_context::_in_context_for::_resource::redis::_new_for_context::base_repository::BaseRepository as ApplicationUserRegistrationConfirmationTokenBaseRepository;
use crate::infrastructure_layer::service::_in_context_for::_resource::_new_for_context::aggregate_connection_pool::AggregateConnectionPool;
use crate::infrastructure_layer::service::_in_context_for::_resource::_new_for_context::connection_extractor::ConnectionExtractor;
use crate::infrastructure_layer::service::_in_context_for::domain_layer::entity::entity::application_user::_new_for_context::email_sender::EmailSender;
use crate::presentation_layer::data_transfer_object::request::_in_context_for::presentation_layer::service::actix_web_component::request_handler::application_programming_interface::version_1::mobile::_in_context_for::domain_layer::entity::entity::application_user::_new_for_context::authorization::_new_for_context::resend_email_for_register::request::Request;
use redis::Connection;
use std::sync::Arc;

pub struct Handler;

impl Handler {
    pub fn handle(aggregate_connection_pool: Arc<AggregateConnectionPool>, request: Request) -> Result<(), BaseError> { // TODO сделать На Редисе механизм для невозможности почстоянно отравки емэйла. (Сохранять, если отправлено, и проверять, что отпрпавили. удалять по времени)
        if let Some(application_user_pre_confirmed) = ApplicationUserPreConfirmedBaseRepository::get_by_application_user_email(
            &*ConnectionExtractor::get_postgresqlxxxdelete_connection(&aggregate_connection_pool)?, &Email::new(request.get_application_user_email())
        )? 
        {
            let application_user_registration_confirmation_token: ApplicationUserRegistrationConfirmationToken<'_>;

            let connection: &'_ mut Connection = &mut *ConnectionExtractor::get_redis_connection(&aggregate_connection_pool)?;

            match ApplicationUserRegistrationConfirmationTokenBaseRepository::get_by_application_user_pre_confirmed_id(connection, application_user_pre_confirmed.get_id()?)? {
                Some(existing_application_user_registration_confirmation_token) => {
                    application_user_registration_confirmation_token = existing_application_user_registration_confirmation_token;

                    ApplicationUserRegistrationConfirmationTokenBaseRepository::update_expiration_time(connection, &application_user_registration_confirmation_token)?;
                },
                None => {
                    application_user_registration_confirmation_token = ApplicationUserRegistrationConfirmationTokenFactory::new_from_application_user_pre_confirmed(&application_user_pre_confirmed)?;

                    ApplicationUserRegistrationConfirmationTokenBaseRepository::create(connection, &application_user_registration_confirmation_token)?;
                }
            }
            
            EmailSender::send_application_user_registration_confirmation_token(&application_user_registration_confirmation_token)?;

            return Ok(());
        }

        return Err(BaseError::EntityError(EntityError::ApplicationUserPreConfirmedError(ApplicationUserPreConfirmedError::NotFound)));
    }
}