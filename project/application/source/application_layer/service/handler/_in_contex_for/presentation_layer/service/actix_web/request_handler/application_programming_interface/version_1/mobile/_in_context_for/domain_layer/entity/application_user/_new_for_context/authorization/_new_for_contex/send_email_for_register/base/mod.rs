use crate::domain_layer::error::entity_error::_component::_in_context_for::domain_layer::entity::application_user_registration_confirmation_token::_new_for_context::application_user_registration_confirmation_token_error::ApplicationUserRegistrationConfirmationTokenError;
use crate::domain_layer::error::entity_error::entity_error::EntityError;
use crate::domain_layer::repository::data_provider::_in_context_for::domain_layer::entity::application_user_registration_confirmation_token::_new_for_context::_in_context_for::_resource::redis::_new_for_context::base_trait::BaseTraitXXXxDelete as ApplicationUserRegistrationConfirmationTokenDataProviderRedisTrait;
use crate::domain_layer::repository::state_manager::_in_context_for::domain_layer::entity::application_user_registration_confirmation_token::_new_for_context::_in_context_for::_resource::redis::_new_for_context::base_trait::BaseTraitXXXxDelete as ApplicationUserRegistrationConfirmationTokenStateManagerRedisTrait;
use crate::domain_layer::service::_in_context_for::domain_layer::entity::application_user::_new_for_context::email_sender_trait::EmailSenderTrait;
use crate::infrastructure_layer::error::base_error::base_error::BaseError;
use crate::infrastructure_layer::repository::data_provider::_in_context_for::domain_layer::entity::application_user_registration_confirmation_token::_new_for_context::_in_context_for::_resource::redis::_new_for_context::base::Base as ApplicationUserRegistrationConfirmationTokenDataProviderRedis;
use crate::infrastructure_layer::repository::state_manager::_in_context_for::domain_layer::entity::application_user_registration_confirmation_token::_new_for_context::_in_context_for::_resource::redis::_new_for_context::base::Base as ApplicationUserRegistrationConfirmationTokenStateManagerRedis;
use crate::infrastructure_layer::service::_in_context_for::domain_layer::entity::application_user::_new_for_context::email_sender::EmailSender;
use crate::infrastructure_layer::service::_in_context_for::infrastructure_layer::repository::_new_for_context::aggregate_connection_pool::AggregateConnectionPoolXXXxDELETE;
use crate::infrastructure_layer::service::_in_context_for::infrastructure_layer::repository::_new_for_context::connection_extractor::ConnectionExtractorXXXxDelete;
use crate::presentation_layer::data_transfer_object::request::_in_context_for::presentation_layer::service::actix_web::request_handler::application_programming_interface::version_1::mobile::_in_context_for::domain_layer::entity::application_user::_new_for_context::authorization::_new_for_context::send_email_for_register::base::Base as Request;
use std::sync::Arc;

pub struct Base;

impl Base {
    pub fn handle(
        aggregate_connection_pool: Arc<AggregateConnectionPoolXXXxDELETE>,
        request: Request
    ) -> Result<(), BaseError> { // TODO  TODO  TODO  TODO сделать На Редисе механизм для невозможности почстоянно отравки емэйла. (Сохранять, если отправлено, и проверять, что отпрпавили. удалять по времени)
        let application_user_email = request.into_inner();

        let redis_connection = &mut *ConnectionExtractorXXXxDelete::get_redis_connection(&aggregate_connection_pool)?;

        match ApplicationUserRegistrationConfirmationTokenDataProviderRedis::find_by_application_user_emailXXXxDelete(
            redis_connection, application_user_email.as_str()
        )? {
            Some(application_user_registration_confirmation_token) => {
                ApplicationUserRegistrationConfirmationTokenStateManagerRedis::update_expiration_timeXXXxDelete(
                    redis_connection, &application_user_registration_confirmation_token
                )?;

                EmailSender::send_application_user_registration_confirmation_token(
                    application_user_registration_confirmation_token.get_value(),
                    application_user_email.as_str()
                )?;
        
                return Ok(());
            },
            None => {
                return Err(
                    BaseError::EntityError {
                        entity_error: EntityError::ApplicationUserRegistrationConfirmationTokenError {
                            application_user_registration_confirmation_token_error: ApplicationUserRegistrationConfirmationTokenError::NotFound
                        }
                    }
                );
            }
        }
    }
}