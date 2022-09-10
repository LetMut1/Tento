use bb8_redis::RedisConnectionManager;
use bb8::Pool;
use crate::application_layer::data::data_transfer_object::_in_context_for::application_layer::functionality::service::action_handler::_new_for_context::action_handler_result::ActionHandlerResult;
use crate::application_layer::data::data_transfer_object::_in_context_for::application_layer::functionality::service::action_handler::_new_for_context::entity_workflow_exception::_component::_in_context_for::domain_layer::data::entity::application_user_access_token::_new_for_context::application_user_access_token_workflow_exception::ApplicationUserAccessTokenWorkflowException;
use crate::application_layer::data::data_transfer_object::_in_context_for::application_layer::functionality::service::action_handler::_new_for_context::entity_workflow_exception::_component::_in_context_for::domain_layer::data::entity::application_user_access_refresh_token::_new_for_context::application_user_access_refresh_token_workflow_exception::ApplicationUserAccessRefreshTokenWorkflowException;
use crate::application_layer::data::data_transfer_object::action_handler_incoming_data::_in_context_for::application_layer::functionality::service::action_handler::_in_context_for::presentation_layer::functionality::service::controller::mobile::version_1::_in_context_for::domain_layer::data::entity::application_user::_new_for_context::authorization::_new_for_context::refresh_application_user_access_token::base::_new_for_context::base::Base as ActionHandlerIncomingData;
use crate::application_layer::data::data_transfer_object::action_handler_outcoming_data::_in_context_for::application_layer::functionality::service::action_handler::_in_context_for::presentation_layer::functionality::service::controller::mobile::version_1::_in_context_for::domain_layer::data::entity::application_user::_new_for_context::authorization::_new_for_context::refresh_application_user_access_token::base::_new_for_context::base::Base as ActionHandlerOutcomingData;
use crate::domain_layer::functionality::service::_in_context_for::domain_layer::data::entity::application_user_access_token::_new_for_context::expiration_time_resolver::ExpirationTimeResolver;
use crate::domain_layer::functionality::service::_in_context_for::domain_layer::data::entity::application_user_access_token::_new_for_context::serialization_form_resolver::SerializationFormResolver;
use crate::domain_layer::functionality::service::_in_context_for::domain_layer::data::entity::application_user_access_refresh_token::_new_for_context::encoder::Encoder;
use crate::domain_layer::functionality::service::_in_context_for::domain_layer::data::entity::application_user_access_refresh_token::_new_for_context::refresher::Refresher;
use crate::domain_layer::functionality::service::factory::_in_context_for::domain_layer::data::entity::application_user_access_token::_new_for_context::base::Base as ApplicationUserAccessTokenFactory;
use crate::infrastructure_layer::data::data_transfer_object::error_auditor::_component::base_error::_component::run_time_error::_component::resource_error::resource_error::ResourceError;
use crate::infrastructure_layer::data::data_transfer_object::error_auditor::_component::base_error::_component::run_time_error::run_time_error::RunTimeError;
use crate::infrastructure_layer::data::data_transfer_object::error_auditor::_component::base_error::base_error::BaseError;
use crate::infrastructure_layer::data::data_transfer_object::error_auditor::_component::simple_backtrace::_component::backtrace_part::BacktracePart;
use crate::infrastructure_layer::data::data_transfer_object::error_auditor::error_auditor::ErrorAuditor;
use crate::infrastructure_layer::functionality::repository::data_provider::_in_context_for::domain_layer::data::entity::application_user_access_refresh_token::_new_for_context::_in_context_for::_resource::redis::_new_for_context::base::Base as ApplicationUserAccessRefreshTokenDataProviderRedis;
use crate::infrastructure_layer::functionality::service::_in_context_for::domain_layer::data::entity::application_user_access_refresh_token::_new_for_context::repository_proxy::RepositoryProxy;
use crate::infrastructure_layer::functionality::service::environment_configuration_resolver::EnvironmentConfigurationResolver;

pub struct Base;

impl Base {
    pub async fn handle<'a>(
        environment_configuration_resolver: &'a EnvironmentConfigurationResolver,
        redis_connection_pool: Pool<RedisConnectionManager>,
        action_handler_incoming_data: ActionHandlerIncomingData
    ) -> Result<ActionHandlerResult<ActionHandlerOutcomingData>, ErrorAuditor> {
        let (
            application_user_access_token_web_form,
            application_user_access_refresh_token_web_form
        ) = action_handler_incoming_data.into_inner();

        match SerializationFormResolver::deserialize(environment_configuration_resolver, application_user_access_token_web_form.as_str()) {
            Ok(application_user_access_token) => {
                match ExpirationTimeResolver::is_expired(&application_user_access_token) {
                    Ok(is_expired) => {
                        if is_expired {        // TODO TODO TODO TODO СДелать интервал, когда можео менять. На 3 часа раньше, чем срок экспирации, например
                            match redis_connection_pool.get().await {
                                Ok(mut redis_pooled_connection) => {
                                    let connection = &mut *redis_pooled_connection;

                                    match ApplicationUserAccessRefreshTokenDataProviderRedis::find_by_application_user_id_and_application_user_log_in_token_device_id(
                                        connection, application_user_access_token.get_application_user_id(), application_user_access_token.get_application_user_log_in_token_device_id()
                                    ).await {
                                        Ok(application_user_access_refresh_token) => {
                                            if let Some(mut application_user_access_refresh_token_) = application_user_access_refresh_token {
                                                match Encoder::is_valid(environment_configuration_resolver, &application_user_access_refresh_token_, application_user_access_refresh_token_web_form.as_str()) {
                                                    Ok(is_valid) => {
                                                        if is_valid && application_user_access_token.get_id().as_bytes() == application_user_access_refresh_token_.get_application_user_access_token_id().as_bytes() {
                                                            Refresher::refresh(&mut application_user_access_refresh_token_);

                                                            if let Err(mut error) = RepositoryProxy::update(connection, &application_user_access_refresh_token_).await {
                                                                error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                                                                return Err(error);
                                                            }

                                                            match ApplicationUserAccessTokenFactory::create_from_application_user_access_refresh_token(&application_user_access_refresh_token_) {
                                                                Ok(ref new_application_user_access_token) => {
                                                                    match SerializationFormResolver::serialize(environment_configuration_resolver, new_application_user_access_token) {
                                                                        Ok(new_application_user_access_token_) => {
                                                                            match Encoder::encode(environment_configuration_resolver, &application_user_access_refresh_token_) {
                                                                                Ok(new_application_user_access_refresh_token) => {
                                                                                    return Ok(ActionHandlerResult::new_with_action_handler_outcoming_data(ActionHandlerOutcomingData::new(new_application_user_access_token_, new_application_user_access_refresh_token)));
                                                                                }
                                                                                Err(mut error) => {
                                                                                    error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                                                                                    return Err(error);
                                                                                }
                                                                            }
                                                                        }
                                                                        Err(mut error) => {
                                                                            error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                                                                            return Err(error);
                                                                        }
                                                                    }
                                                                }
                                                                Err(mut error) => {
                                                                    error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                                                                    return Err(error);
                                                                }
                                                            }
                                                        }

                                                        return Err(
                                                            ErrorAuditor::new(
                                                                BaseError::InvalidArgumentError,
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

                                            return Ok(ActionHandlerResult::new_with_application_user_access_refresh_token_workflow_exception(ApplicationUserAccessRefreshTokenWorkflowException::NotFound));
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
                                            BaseError::RunTimeError { run_time_error: RunTimeError::ResourceError { resource_error: ResourceError::ConnectionPoolRedisError { bb8_redis_error: error } } },
                                            BacktracePart::new(line!(), file!(), None)
                                        )
                                    );
                                }
                            }
                        }

                        return Ok(ActionHandlerResult::new_with_application_user_access_token_workflow_exception(ApplicationUserAccessTokenWorkflowException::NotExpired));
                    }
                    Err(mut error) => {
                        error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                        return Err(error);
                    }
                }
            }
            Err(mut error) => {
                error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                return Err(error);
            }
        }
    }
}