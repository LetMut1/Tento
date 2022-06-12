use bb8_redis::RedisConnectionManager;
use bb8::Pool;
use crate::application_layer::data_transfer_object::_in_context_for::application_layer::service::action_handler::_new_for_context::action_handler_result::ActionHandlerResult;
use crate::application_layer::data_transfer_object::_in_context_for::application_layer::service::action_handler::_new_for_context::entity_workflow_exception::_component::_in_context_for::domain_layer::entity::application_user_log_in_token::_new_for_context::application_user_log_in_token_workflow_exception::ApplicationUserLogInTokenWorkflowException;
use crate::application_layer::data_transfer_object::action_handler_incoming_data::_in_context_for::application_layer::service::action_handler::_in_context_for::presentation_layer::service::controller::mobile::version_1::_in_context_for::domain_layer::entity::application_user::_new_for_context::authorization::_new_for_context::log_in_by_last_step::base::_new_for_context::base::Base as ActionHandlerIncomingData;
use crate::application_layer::data_transfer_object::action_handler_outcoming_data::_in_context_for::application_layer::service::action_handler::_in_context_for::presentation_layer::service::controller::mobile::version_1::_in_context_for::domain_layer::entity::application_user::_new_for_context::authorization::_new_for_context::log_in_by_last_step::base::_new_for_context::base::Base as ActionHandlerOutcomingData;
use crate::domain_layer::entity::application_user_log_in_token::ApplicationUserLogInToken;
use crate::domain_layer::entity::json_access_web_token_black_list::JsonAccessWebTokenBlackList;
use crate::domain_layer::service::_in_context_for::domain_layer::entity::application_user_log_in_token::_new_for_context::wrong_enter_tries_quantity_incrementor::WrongEnterTriesQuantityIncrementor;
use crate::domain_layer::service::_in_context_for::domain_layer::entity::json_access_web_token::_new_for_context::serialization_form_resolver::SerializationFormResolver;
use crate::domain_layer::service::_in_context_for::domain_layer::entity::json_refresh_web_token::_new_for_context::encoder::Encoder;
use crate::domain_layer::service::factory::_in_context_for::domain_layer::entity::json_access_web_token::_new_for_context::base::Base as JsonAccessWebTokenFactory;
use crate::domain_layer::service::factory::_in_context_for::domain_layer::entity::json_refresh_web_token::_new_for_context::base::Base as JsonRefreshWebTokenFactory;
use crate::infrastructure_layer::error::error_auditor::_component::base_error::_component::run_time_error::_component::resource_error::resource_error::ResourceError;
use crate::infrastructure_layer::error::error_auditor::_component::base_error::_component::run_time_error::run_time_error::RunTimeError;
use crate::infrastructure_layer::error::error_auditor::_component::base_error::base_error::BaseError;
use crate::infrastructure_layer::error::error_auditor::_component::simple_backtrace::_component::backtrace_part::BacktracePart;
use crate::infrastructure_layer::error::error_auditor::error_auditor::ErrorAuditor;
use crate::infrastructure_layer::repository::data_provider::_in_context_for::domain_layer::entity::application_user_log_in_token::_new_for_context::_in_context_for::_resource::redis::_new_for_context::base::Base as ApplicationUserLogInTokenDataProviderRedis;
use crate::infrastructure_layer::repository::data_provider::_in_context_for::domain_layer::entity::json_refresh_web_token::_new_for_context::_in_context_for::_resource::redis::_new_for_context::base::Base as JsonRefreshWebTokenDataProviderRedis;
use crate::infrastructure_layer::repository::state_manager::_in_context_for::domain_layer::entity::application_user_log_in_token::_new_for_context::_in_context_for::_resource::redis::_new_for_context::base::Base as ApplicationUserLogInTokenStateManagerRedis;
use crate::infrastructure_layer::repository::state_manager::_in_context_for::domain_layer::entity::json_access_web_token_black_list::_new_for_context::_in_context_for::_resource::redis::_new_for_context::base::Base as JsonAccessWebTokenBlackListStateManagerRedis;
use crate::infrastructure_layer::service::_in_context_for::domain_layer::entity::json_refresh_web_token::_new_for_context::repository_proxy::RepositoryProxy;
use crate::infrastructure_layer::service::environment_configuration_resolver::EnvironmentConfigurationResolver;

pub struct Base;

impl Base {
    pub async fn handle<'a>(
        environment_configuration_resolver: &'a EnvironmentConfigurationResolver,
        redis_connection_pool: Pool<RedisConnectionManager>,        // TODO  TODO  TODO  TODO  TODO МОжет ли хакер войти на этом шаге, если пользователь сделал первый шаг.
        action_handler_incoming_data: ActionHandlerIncomingData
    ) -> Result<ActionHandlerResult<ActionHandlerOutcomingData>, ErrorAuditor> {   // TODO сделать На Редисе механизм для невозможности почстоянно отравки емэйла. (Сохранять, если отправлено, и проверять, что отпрпавили. удалять по времени)
        let (
            application_user_id,
            application_user_log_in_token_device_id,  // TODO ПРоверить все входящие значения application_user_log_in_token_device_id нв формат. Формата может не быть. Нужно определиться, есть ли формат, напримре, UUID
            application_user_log_in_token_value
        ) = action_handler_incoming_data.into_inner();

        match redis_connection_pool.get().await {
            Ok(mut pooled_connection) => {
                let redis_connection = &mut *pooled_connection;

                match ApplicationUserLogInTokenDataProviderRedis::find_by_application_user_id_and_device_id(
                    redis_connection, application_user_id, application_user_log_in_token_device_id.as_str()
                ).await {
                    Ok(application_user_log_in_token) => {
                        if let Some(mut application_user_log_in_token_) = application_user_log_in_token {
                            if application_user_log_in_token_.get_value() == application_user_log_in_token_value.as_str() {
                                match JsonRefreshWebTokenDataProviderRedis::find_by_application_user_id_and_application_user_log_in_token_device_id(
                                    redis_connection, application_user_log_in_token_.get_application_user_id(), application_user_log_in_token_.get_device_id()
                                ).await {
                                    Ok(json_refresh_web_token) => {
                                        if let Some(json_refresh_web_token_) = json_refresh_web_token {
                                            if let Err(mut error) = JsonAccessWebTokenBlackListStateManagerRedis::create(
                                                redis_connection, &JsonAccessWebTokenBlackList::new(json_refresh_web_token_.get_json_access_web_token_id())
                                            ).await {
                                                error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));
                        
                                                return Err(error);
                                            }
                        
                                            if let Err(mut error) = RepositoryProxy::delete(redis_connection, &json_refresh_web_token_).await {
                                                error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));
                        
                                                return Err(error);
                                            }
                                        }
                        
                                        let json_refresh_web_token_ = JsonRefreshWebTokenFactory::create_from_id_registry(
                                            application_user_log_in_token_.get_application_user_id(), application_user_log_in_token_.get_device_id()
                                        );
                                        
                                        if let Err(mut error) = ApplicationUserLogInTokenStateManagerRedis::delete(redis_connection, &application_user_log_in_token_).await {
                                            error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));
                        
                                            return Err(error);
                                        }
                        
                                        if let Err(mut error) = RepositoryProxy::create(redis_connection, &json_refresh_web_token_).await {
                                            error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));
                        
                                            return Err(error);
                                        }
                        
                                        match JsonAccessWebTokenFactory::create_from_json_refresh_web_token(&json_refresh_web_token_) {
                                            Ok(ref json_access_web_token) => {
                                                match SerializationFormResolver::serialize(environment_configuration_resolver, json_access_web_token) {
                                                    Ok(json_access_web_token_) => {
                                                        match Encoder::encode(environment_configuration_resolver, &json_refresh_web_token_) {
                                                            Ok(json_refresh_web_token_) => {
                                                                return Ok(ActionHandlerResult::new_with_action_handler_outcoming_data(ActionHandlerOutcomingData::new(json_access_web_token_, json_refresh_web_token_)));
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
                                    Err(mut error) => {
                                        error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));
                        
                                        return Err(error);
                                    }
                                }
                            }
                
                            if let Err(mut error) = WrongEnterTriesQuantityIncrementor::increment(&mut application_user_log_in_token_) {
                                error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));
                
                                return Err(error);
                            }
                
                            if application_user_log_in_token_.get_wrong_enter_tries_quantity() <= ApplicationUserLogInToken::WRONG_ENTER_TRIES_QUANTITY_LIMIT {
                                if let Err(mut error) = ApplicationUserLogInTokenStateManagerRedis::create(redis_connection, &application_user_log_in_token_).await {
                                    error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));
                        
                                    return Err(error);
                                }
                            } else {
                                if let Err(mut error) = ApplicationUserLogInTokenStateManagerRedis::delete(redis_connection, &application_user_log_in_token_).await {
                                    error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));
                        
                                    return Err(error);
                                }
                            }
                            
                            return Ok(ActionHandlerResult::new_with_application_user_log_in_token_workflow_exception(ApplicationUserLogInTokenWorkflowException::InvalidValue));
                        }
                
                        return Ok(ActionHandlerResult::new_with_application_user_log_in_token_workflow_exception(ApplicationUserLogInTokenWorkflowException::NotFound));
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