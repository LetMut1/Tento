use bb8_redis::RedisConnectionManager;
use bb8::Pool;
use crate::application_layer::data::data_transfer_object::_in_context_for::application_layer::functionality::service::action_handler::_new_for_context::action_handler_result::ActionHandlerResult;
use crate::application_layer::data::data_transfer_object::_in_context_for::application_layer::functionality::service::action_handler::_new_for_context::entity_workflow_exception::_component::_in_context_for::domain_layer::data::entity::application_user_access_token::_new_for_context::application_user_access_token_workflow_exception::ApplicationUserAccessTokenWorkflowException;
use crate::application_layer::data::data_transfer_object::_in_context_for::application_layer::functionality::service::action_handler::_new_for_context::entity_workflow_exception::_component::_in_context_for::domain_layer::data::entity::application_user_access_refresh_token::_new_for_context::application_user_access_refresh_token_workflow_exception::ApplicationUserAccessRefreshTokenWorkflowException;
use crate::application_layer::data::data_transfer_object::action_handler_incoming_data::_in_context_for::application_layer::functionality::service::action_handler::_in_context_for::presentation_layer::functionality::service::controller::mobile::version_1::_in_context_for::domain_layer::data::entity::application_user::_new_for_context::authorization::_new_for_context::log_out_from_one_device::base::_new_for_context::base::Base as ActionHandlerIncomingData;
use crate::domain_layer::data::entity::application_user_access_token_black_list::ApplicationUserAccessTokenBlackList;
use crate::infrastructure_layer::data::data_transfer_object::_in_context_for::infrastructure_layer::functionality::service::_in_context_for::domain_layer::data::entity::json_access_web_token::_new_for_context::extractor::_new_for_context::result::Result as ExtractorResult;
use crate::infrastructure_layer::data::data_transfer_object::error_auditor::_component::base_error::_component::run_time_error::_component::resource_error::resource_error::ResourceError;
use crate::infrastructure_layer::data::data_transfer_object::error_auditor::_component::base_error::_component::run_time_error::run_time_error::RunTimeError;
use crate::infrastructure_layer::data::data_transfer_object::error_auditor::_component::base_error::base_error::BaseError;
use crate::infrastructure_layer::data::data_transfer_object::error_auditor::_component::simple_backtrace::_component::backtrace_part::BacktracePart;
use crate::infrastructure_layer::data::data_transfer_object::error_auditor::error_auditor::ErrorAuditor;
use crate::infrastructure_layer::functionality::repository::data_provider::_in_context_for::domain_layer::data::entity::application_user_access_refresh_token::_new_for_context::_in_context_for::_resource::redis::_new_for_context::base::Base as JsonRefreshWebTokenDataProviderRedis;
use crate::infrastructure_layer::functionality::repository::state_manager::_in_context_for::domain_layer::data::entity::application_user_access_token_black_list::_new_for_context::_in_context_for::_resource::redis::_new_for_context::base::Base as JsonAccessWebTokenBlackListStateManagerRedis;
use crate::infrastructure_layer::functionality::service::_in_context_for::domain_layer::data::entity::application_user_access_token::_new_for_context::extractor::Extractor;
use crate::infrastructure_layer::functionality::service::_in_context_for::domain_layer::data::entity::application_user_access_refresh_token::_new_for_context::repository_proxy::RepositoryProxy;
use crate::infrastructure_layer::functionality::service::environment_configuration_resolver::EnvironmentConfigurationResolver;

pub struct Base;

impl Base {
    pub async fn handle<'a>(
        environment_configuration_resolver: &'a EnvironmentConfigurationResolver,
        redis_connection_pool: Pool<RedisConnectionManager>,
        action_handler_incoming_data: ActionHandlerIncomingData
    ) -> Result<ActionHandlerResult<()>, ErrorAuditor> {
        match redis_connection_pool.get().await {
            Ok(mut redis_pooled_connection) => {
                let connection = &mut *redis_pooled_connection;

                let application_user_access_token_web_form = action_handler_incoming_data.into_inner();
                match Extractor::extract(environment_configuration_resolver, application_user_access_token_web_form.as_str(), connection).await {
                    Ok(result) => {
                        match result {
                            ExtractorResult::ApplicationUserAccessToken { application_user_access_token } => {
                                match JsonRefreshWebTokenDataProviderRedis::find_by_application_user_id_and_application_user_log_in_token_device_id(
                                    connection, application_user_access_token.get_application_user_id(), application_user_access_token.get_application_user_log_in_token_device_id()
                                ).await {
                                    Ok(application_user_access_refresh_token) => {
                                        if let Some(application_user_access_refresh_token_) = application_user_access_refresh_token {
                                            if let Err(mut error) = RepositoryProxy::delete(connection, &application_user_access_refresh_token_).await {
                                                error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                                                return Err(error);
                                            }

                                            if let Err(mut error) = JsonAccessWebTokenBlackListStateManagerRedis::create(
                                                connection, &ApplicationUserAccessTokenBlackList::new(application_user_access_token.get_id())
                                            ).await {
                                                error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                                                return Err(error);
                                            }

                                            return Ok(ActionHandlerResult::new_with_action_handler_outcoming_data(()));
                                        }

                                        return Ok(ActionHandlerResult::new_with_application_user_access_refresh_token_workflow_exception(ApplicationUserAccessRefreshTokenWorkflowException::NotFound));
                                    }
                                    Err(mut error) => {
                                        error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                                        return Err(error);
                                    }
                                }
                            }
                            ExtractorResult::ApplicationUserAccessTokenAlreadyExpired => {
                                return Ok(ActionHandlerResult::new_with_application_user_access_token_workflow_exception(ApplicationUserAccessTokenWorkflowException::AlreadyExpired));
                            }
                            ExtractorResult::ApplicationUserAccessTokenInApplicationUserAccessTokenBlackList => {
                                return Ok(ActionHandlerResult::new_with_application_user_access_token_workflow_exception(ApplicationUserAccessTokenWorkflowException::InApplicationUserAccessTokenBlackList));
                            }
                        }
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
}