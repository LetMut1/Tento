use bb8_redis::RedisConnectionManager;
use bb8::Pool;
use crate::application_layer::data_transfer_object::_in_context_for::application_layer::service::action_handler::_new_for_context::action_handler_result::ActionHandlerResult;
use crate::application_layer::data_transfer_object::_in_context_for::application_layer::service::action_handler::_new_for_context::entity_workflow_exception::_component::_in_context_for::domain_layer::entity::json_access_web_token::_new_for_context::json_access_web_token_workflow_exception::JsonAccessWebTokenWorkflowException;
use crate::application_layer::data_transfer_object::_in_context_for::application_layer::service::action_handler::_new_for_context::entity_workflow_exception::_component::_in_context_for::domain_layer::entity::json_refresh_web_token::_new_for_context::json_refresh_web_token_workflow_exception::JsonRefreshWebTokenWorkflowException;
use crate::application_layer::data_transfer_object::action_handler_incoming_data::_in_context_for::application_layer::service::action_handler::_in_context_for::presentation_layer::service::controller::mobile::version_1::_in_context_for::domain_layer::entity::application_user::_new_for_context::authorization::_new_for_context::log_out_from_all_devices::base::_new_for_context::base::Base as ActionHandlerIncomingData;
use crate::domain_layer::entity::json_access_web_token_black_list::JsonAccessWebTokenBlackList;
use crate::infrastructure_layer::data_transfer_object::_in_context_for::infrastructure_layer::service::_in_context_for::domain_layer::entity::json_access_web_token::_new_for_context::extractor::_new_for_context::result::Result as ExtractorResult;
use crate::infrastructure_layer::error::error_auditor::_component::base_error::_component::run_time_error::_component::resource_error::resource_error::ResourceError;
use crate::infrastructure_layer::error::error_auditor::_component::base_error::_component::run_time_error::run_time_error::RunTimeError;
use crate::infrastructure_layer::error::error_auditor::_component::base_error::base_error::BaseError;
use crate::infrastructure_layer::error::error_auditor::_component::simple_backtrace::_component::backtrace_part::BacktracePart;
use crate::infrastructure_layer::error::error_auditor::error_auditor::ErrorAuditor;
use crate::infrastructure_layer::repository::state_manager::_in_context_for::domain_layer::entity::json_access_web_token_black_list::_new_for_context::_in_context_for::_resource::redis::_new_for_context::base::Base as JsonAccessWebTokenBlackListStateManagerRedis;
use crate::infrastructure_layer::service::_in_context_for::domain_layer::entity::json_access_web_token::_new_for_context::extractor::Extractor;
use crate::infrastructure_layer::service::_in_context_for::domain_layer::entity::json_refresh_web_token::_new_for_context::repository_proxy::RepositoryProxy;
use crate::infrastructure_layer::service::environment_configuration_resolver::EnvironmentConfigurationResolver;

pub struct Base;

impl Base {
    pub async fn handle<'a>(             // TODO TODO TODO УДАляются ли АккессТокены все при массовом разлогине? Если не удаляются, можно просто при Ектракте АккессТокена использовать проверку на наличие рефреша, если нет, значит произошел разлогин.
        environment_configuration_resolver: &'a EnvironmentConfigurationResolver,
        redis_connection_pool: Pool<RedisConnectionManager>,
        action_handler_incoming_data: ActionHandlerIncomingData
    ) -> Result<ActionHandlerResult<()>, ErrorAuditor> {
        match redis_connection_pool.get().await {
            Ok(mut redis_pooled_connection) => {
                let connection = &mut *redis_pooled_connection;

                let json_access_web_token = action_handler_incoming_data.into_inner();
                match Extractor::extract(environment_configuration_resolver, json_access_web_token.as_str(), connection).await {
                    Ok(result) => {
                        match result {
                            ExtractorResult::JsonAccessWebToken { json_access_web_token: json_access_web_token_ } => {
                                match RepositoryProxy::get_by_application_user_id(connection, json_access_web_token_.get_application_user_id()).await {
                                    Ok(json_refresh_web_token_registry) => {
                                        if let Some(json_refresh_web_token_registry_) = json_refresh_web_token_registry {
                                            '_a: for json_refresh_web_token in json_refresh_web_token_registry_.iter() {
                                                if let Err(mut error) = RepositoryProxy::delete(connection, json_refresh_web_token).await {
                                                    error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));
                        
                                                    return Err(error);
                                                }
                                
                                                if let Err(mut error) = JsonAccessWebTokenBlackListStateManagerRedis::create(connection, &JsonAccessWebTokenBlackList::new(json_access_web_token_.get_id())).await {
                                                    error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));
                        
                                                    return Err(error);
                                                }
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
                            ExtractorResult::JsonAccessWebTokenAlreadyExpired => {
                                return Ok(ActionHandlerResult::new_with_json_access_web_token_workflow_exception(JsonAccessWebTokenWorkflowException::AlreadyExpired));
                            }
                            ExtractorResult::JsonAccessWebTokenInJsonAccessWebTokenBlackList => {
                                return Ok(ActionHandlerResult::new_with_json_access_web_token_workflow_exception(JsonAccessWebTokenWorkflowException::InJsonAccessWebTokenBlackList));
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