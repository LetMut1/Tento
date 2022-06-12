use bb8_redis::RedisConnectionManager;
use bb8::Pool;
use crate::application_layer::data_transfer_object::_in_context_for::application_layer::service::action_handler::_new_for_context::action_handler_result::ActionHandlerResult;
use crate::application_layer::data_transfer_object::_in_context_for::application_layer::service::action_handler::_new_for_context::entity_workflow_exception::_component::_in_context_for::domain_layer::entity::json_refresh_web_token::_new_for_context::json_refresh_web_token_workflow_exception::JsonRefreshWebTokenWorkflowException;
use crate::application_layer::data_transfer_object::action_handler_incoming_data::_in_context_for::application_layer::service::action_handler::_in_context_for::presentation_layer::service::controller::mobile::version_1::_in_context_for::domain_layer::entity::application_user::_new_for_context::authorization::_new_for_context::log_out_from_one_device::base::_new_for_context::base::Base as ActionHandlerIncomingData;
use crate::domain_layer::entity::json_access_web_token_black_list::JsonAccessWebTokenBlackList;
use crate::infrastructure_layer::error::error_auditor::_component::base_error::_component::run_time_error::_component::resource_error::resource_error::ResourceError;
use crate::infrastructure_layer::error::error_auditor::_component::base_error::_component::run_time_error::run_time_error::RunTimeError;
use crate::infrastructure_layer::error::error_auditor::_component::base_error::base_error::BaseError;
use crate::infrastructure_layer::error::error_auditor::_component::simple_backtrace::_component::backtrace_part::BacktracePart;
use crate::infrastructure_layer::error::error_auditor::error_auditor::ErrorAuditor;
use crate::infrastructure_layer::repository::data_provider::_in_context_for::domain_layer::entity::json_refresh_web_token::_new_for_context::_in_context_for::_resource::redis::_new_for_context::base::Base as JsonRefreshWebTokenDataProviderRedis;
use crate::infrastructure_layer::repository::state_manager::_in_context_for::domain_layer::entity::json_access_web_token_black_list::_new_for_context::_in_context_for::_resource::redis::_new_for_context::base::Base as JsonAccessWebTokenBlackListStateManagerRedis;
use crate::infrastructure_layer::service::_in_context_for::domain_layer::entity::json_access_web_token::_new_for_context::extractor::Extractor;
use crate::infrastructure_layer::service::_in_context_for::domain_layer::entity::json_refresh_web_token::_new_for_context::repository_proxy::RepositoryProxy;
use crate::infrastructure_layer::service::environment_configuration_resolver::EnvironmentConfigurationResolver;

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

                let json_access_web_token = action_handler_incoming_data.into_inner();
                match Extractor::extract(environment_configuration_resolver, json_access_web_token.as_str(), connection).await {
                    Ok(json_access_web_token_) => {
                        match JsonRefreshWebTokenDataProviderRedis::find_by_application_user_id_and_application_user_log_in_token_device_id(
                            connection, json_access_web_token_.get_application_user_id(), json_access_web_token_.get_application_user_log_in_token_device_id()
                        ).await {
                            Ok(json_refresh_web_token) => {
                                if let Some(json_refresh_web_token_) = json_refresh_web_token {
                                    if let Err(mut error) = RepositoryProxy::delete(connection, &json_refresh_web_token_).await {
                                        error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));
                
                                        return Err(error);
                                    }
                        
                                    if let Err(mut error) = JsonAccessWebTokenBlackListStateManagerRedis::create(connection, &JsonAccessWebTokenBlackList::new(json_access_web_token_.get_id())).await {
                                        error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));
                
                                        return Err(error);
                                    }
                        
                                    return Ok(ActionHandlerResult::new_with_action_handler_outcoming_data(()));
                                }
                        
                                return Ok(ActionHandlerResult::new_with_json_refresh_web_token_workflow_exception(JsonRefreshWebTokenWorkflowException::NotFound));
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