use crate::domain_layer::entity::entity::application_user_registration_confirmation_token::application_user_registration_confirmation_token::ApplicationUserRegistrationConfirmationToken;
use crate::domain_layer::entity::entity::application_user::_core::email::Email;
use crate::domain_layer::entity::entity::pre_confirmed_application_user::pre_confirmed_application_user::PreConfirmedApplicationUser;
use crate::domain_layer::error::base_error::_core::entity_error::_core::_in_context_for::entity::entity::application_user::_new_for_context::application_user_error::ApplicationUserError;
use crate::domain_layer::error::base_error::_core::entity_error::_core::_in_context_for::entity::entity::pre_confirmed_application_user::_new_for_context::pre_confirmed_application_user_error::PreConfirmedApplicationUserError;
use crate::domain_layer::error::base_error::_core::entity_error::entity_error::EntityError;
use crate::domain_layer::error::base_error::base_error::BaseError;
use crate::domain_layer::service::_in_context_for::domain_layer::entity::entity::application_user::_new_for_context::email_sender_trait::EmailSenderTrait;
use crate::infrastructure_layer::repository::_in_context_for::entity::entity::application_user_registration_confirmation_token::_new_for_context::_in_context_for::_resource::redis::_new_for_context::base_repository::BaseRepository as ApplicationUserRegistrationConfirmationTokenBaseRepository;
use crate::infrastructure_layer::repository::_in_context_for::entity::entity::application_user::_new_for_context::_in_context_for::_resource::postgresql::_new_for_context::base_repository::BaseRepository as ApplicationUserBaseRepository;
use crate::infrastructure_layer::repository::_in_context_for::entity::entity::pre_confirmed_application_user::_new_for_context::_in_context_for::_resource::postgresql::_new_for_context::base_repository::BaseRepository as PreConfirmedApplicationUserBaseRepository;
use crate::infrastructure_layer::service::_in_context_for::domain_layer::entity::entity::application_user::_new_for_context::email_sender::EmailSender;
use crate::infrastructure_layer::utility::_in_context_for::_resource::_new_for_context::aggregate_connection_pool::AggregateConnectionPool;
use crate::infrastructure_layer::utility::_in_context_for::_resource::_new_for_context::connection_extractor::ConnectionExtractor;
use crate::presentation_layer::data_transfer_object::request::_in_context_for::presentation_layer::actix_web_component::request_handler::api::version1::mobile::_in_context_for::domain_layer::entity::entity::application_user::_new_for_context::authorization::_new_for_context::pre_register::request::Request;
use diesel::PgConnection as PostgresqlConnection;
use std::sync::Arc;

pub struct Handler;

impl Handler {
    pub fn handle(aggregate_connection_pool: Arc<AggregateConnectionPool>, request: Request) -> Result<(), BaseError> {
        let application_user_email: Email = Email::new(request.get_application_user_email());
        if application_user_email.is_valid()? {
            let postgresql_connection: &'_ PostgresqlConnection = &*ConnectionExtractor::get_postgresql_connection(&aggregate_connection_pool)?;

            if !PreConfirmedApplicationUserBaseRepository::is_exist_by_application_user_email(postgresql_connection, &application_user_email)? {
                if !ApplicationUserBaseRepository::is_exist_by_email(postgresql_connection, &application_user_email)? {
                    let pre_confirmed_application_user: PreConfirmedApplicationUser = PreConfirmedApplicationUser::new(application_user_email);  

                    let application_user_registration_confirmation_token: ApplicationUserRegistrationConfirmationToken<'_> =
                    ApplicationUserRegistrationConfirmationToken::new(&pre_confirmed_application_user)?;
                    
                    ApplicationUserRegistrationConfirmationTokenBaseRepository::create(
                        &mut *ConnectionExtractor::get_redis_connection(&aggregate_connection_pool)?, &application_user_registration_confirmation_token
                    )?;

                    PreConfirmedApplicationUserBaseRepository::create(postgresql_connection, &pre_confirmed_application_user)?;

                    <EmailSender as EmailSenderTrait>::send_application_user_registration_confirmation_token(&application_user_registration_confirmation_token)?;

                    return Ok(());
                }
                
                return Err(BaseError::EntityError(EntityError::ApplicationUserError(ApplicationUserError::AlreadyExist)));
            }
            
            return Err(BaseError::EntityError(EntityError::PreConfirmedApplicationUserError(PreConfirmedApplicationUserError::AlreadyExist)));
        }
        
        return Err(BaseError::EntityError(EntityError::ApplicationUserError(ApplicationUserError::InvalidEmail)));
    }
}