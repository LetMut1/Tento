use bb8_redis::RedisConnectionManager;
use bb8::Pool;
use crate::application_layer::data_transfer_object::action_handler_incoming_data::_in_context_for::application_layer::service::action_handler::_in_context_for::presentation_layer::service::controller::mobile::version_1::_in_context_for::domain_layer::entity::application_user::_new_for_context::authorization::_new_for_context::send_email_for_register::base::_new_for_context::base::Base as ActionHandlerIncomingData;
use crate::domain_layer::error::entity_error::_component::_in_context_for::domain_layer::entity::application_user_registration_confirmation_token::_new_for_context::application_user_registration_confirmation_token_error::ApplicationUserRegistrationConfirmationTokenError;
use crate::domain_layer::error::entity_error::entity_error::EntityError;
use crate::infrastructure_layer::error::error_auditor::_component::error_aggregator::_component::run_time_error::_component::resource_error::resource_error::ResourceError;
use crate::infrastructure_layer::error::error_auditor::_component::error_aggregator::_component::run_time_error::run_time_error::RunTimeError;
use crate::infrastructure_layer::error::error_auditor::_component::error_aggregator::error_aggregator::ErrorAggregator;
use crate::infrastructure_layer::error::error_auditor::_component::simple_backtrace::_component::backtrace_part::BacktracePart;
use crate::infrastructure_layer::error::error_auditor::error_auditor::ErrorAuditor;
use crate::infrastructure_layer::repository::data_provider::_in_context_for::domain_layer::entity::application_user_registration_confirmation_token::_new_for_context::_in_context_for::_resource::redis::_new_for_context::base::Base as ApplicationUserRegistrationConfirmationTokenDataProviderRedis;
use crate::infrastructure_layer::repository::state_manager::_in_context_for::domain_layer::entity::application_user_registration_confirmation_token::_new_for_context::_in_context_for::_resource::redis::_new_for_context::base::Base as ApplicationUserRegistrationConfirmationTokenStateManagerRedis;
use crate::infrastructure_layer::service::_in_context_for::domain_layer::entity::application_user::_new_for_context::email_sender::EmailSender;
use crate::infrastructure_layer::service::environment_configuration_resolver::EnvironmentConfigurationResolver;

pub struct Base;

impl Base {
    pub async fn handle<'a>(
        environment_configuration_resolver: &'a EnvironmentConfigurationResolver,
        redis_connection_pool: Pool<RedisConnectionManager>,
        action_handler_incoming_data: ActionHandlerIncomingData
    ) -> Result<(), ErrorAuditor> { // TODO  TODO  TODO  TODO сделать На Редисе механизм для невозможности почстоянно отравки емэйла. (Сохранять, если отправлено, и проверять, что отпрпавили. удалять по времени)
        let application_user_email = action_handler_incoming_data.into_inner();

        match redis_connection_pool.get().await {
            Ok(mut redis_pooled_connection) => {
                let redis_connection = &mut *redis_pooled_connection;

                match ApplicationUserRegistrationConfirmationTokenDataProviderRedis::find_by_application_user_email(
                    redis_connection, application_user_email.as_str()
                ).await {
                    Ok(application_user_registration_confirmation_token) => {
                        if let Some(application_user_registration_confirmation_token_) = application_user_registration_confirmation_token {
                            if let Err(mut error) = ApplicationUserRegistrationConfirmationTokenStateManagerRedis::update_expiration_time(
                                redis_connection, &application_user_registration_confirmation_token_
                            ).await {
                                error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));
        
                                return Err(error);
                            }
            
                            if let Err(mut error) = EmailSender::send_application_user_registration_confirmation_token(
                                environment_configuration_resolver,
                                application_user_registration_confirmation_token_.get_value(),
                                application_user_email.as_str()
                            ) {
                                error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));
                
                                return Err(error);
                            }
                    
                            return Ok(());
                        }
        
                        return Err(
                            ErrorAuditor::new(
                                ErrorAggregator::EntityError {
                                    entity_error: EntityError::ApplicationUserRegistrationConfirmationTokenError {
                                        application_user_registration_confirmation_token_error: ApplicationUserRegistrationConfirmationTokenError::NotFound
                                    }
                                },
                                BacktracePart::new(line!(), file!(), None)
                            )
                        );
                    }
                    Err(mut error) => {
                        error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));
        
                        return Err(error);
                    }
                }
            }
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        ErrorAggregator::RunTimeError { run_time_error: RunTimeError::ResourceError { resource_error: ResourceError::ConnectionPoolRedisError { bb8_redis_error: error } } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        }
    }
}