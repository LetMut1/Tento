use crate::domain_layer::entity::json_refresh_web_token::JsonRefreshWebToken;
use crate::infrastructure_layer::data_transfer_object::_in_context_for::infrastructure_layer::repository::state_manager::_in_context_for::domain_layer::entity::json_refresh_web_token::_new_for_context::_in_context_for::_resource::redis::_new_for_context::base::_new_for_context::common::Common;
use crate::infrastructure_layer::data_transfer_object::error_auditor::_component::base_error::_component::run_time_error::_component::other_error::OtherError;
use crate::infrastructure_layer::data_transfer_object::error_auditor::_component::base_error::_component::run_time_error::_component::resource_error::resource_error::ResourceError;
use crate::infrastructure_layer::data_transfer_object::error_auditor::_component::base_error::_component::run_time_error::run_time_error::RunTimeError;
use crate::infrastructure_layer::data_transfer_object::error_auditor::_component::base_error::base_error::BaseError;
use crate::infrastructure_layer::data_transfer_object::error_auditor::_component::simple_backtrace::_component::backtrace_part::BacktracePart;
use crate::infrastructure_layer::data_transfer_object::error_auditor::error_auditor::ErrorAuditor;
use crate::infrastructure_layer::service::_in_context_for::infrastructure_layer::repository::_new_for_context::_in_context_for::_resource::redis::_new_for_context::storage_key_resolver::StorageKeyResolver;
use redis::aio::Connection;
use redis::AsyncCommands;

pub struct Base;

impl Base {
    pub async fn find_by_application_user_id_and_application_user_log_in_token_device_id<'a>(
        connection: &'a mut Connection, 
        application_user_id: i64, 
        application_user_log_in_token_device_id: &'a str,
    ) -> Result<Option<JsonRefreshWebToken<'static>>, ErrorAuditor> {
        match connection.get::<String, Option<Vec<u8>>>(StorageKeyResolver::get_5(application_user_id, application_user_log_in_token_device_id)).await {
            Ok(data) => {
                match data {
                    Some(data_) => {
                        match rmp_serde::from_read_ref::<'_, [u8], Common<'static>>(&data_[..]) {
                            Ok(common) => {
                                let (
                                    json_access_web_token_id,
                                    application_user_id,
                                    application_user_log_in_token_device_id,
                                    json_refresh_web_token_obfuscation_value
                                ) = common.into_inner();
                        
                                let json_refresh_web_token = JsonRefreshWebToken::new(
                                    json_access_web_token_id.into_owned(),
                                    application_user_id,
                                    application_user_log_in_token_device_id,
                                    json_refresh_web_token_obfuscation_value.into_owned()
                                );
                
                                return Ok(Some(json_refresh_web_token));
                            }
                            Err(error) => {
                                return Err(
                                    ErrorAuditor::new(
                                        BaseError::RunTimeError { run_time_error: RunTimeError::OtherError { other_error: OtherError::new(error) } },
                                        BacktracePart::new(line!(), file!(), None)
                                    )
                                );
                            }
                        }
                    }
                    None => {
                        return Ok(None);
                    }
                }
            }
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RunTimeError { run_time_error: RunTimeError::ResourceError { resource_error: ResourceError::RedisError { redis_error: error } } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        }
    }

    pub async fn find_by_application_user_id_and_application_user_log_in_token_device_id_registry<'a>(
        connection: &'a mut Connection, 
        application_user_id: i64, 
        application_user_log_in_token_device_id_registry: Vec<String>
    ) -> Result<Option<Vec<JsonRefreshWebToken<'static>>>, ErrorAuditor> {
        let mut json_refresh_web_token_registry: Vec<JsonRefreshWebToken<'_>> = vec![];

        '_a: for application_user_log_in_token_device_id in application_user_log_in_token_device_id_registry.into_iter() {
            match Self::find_by_application_user_id_and_application_user_log_in_token_device_id(
                connection, application_user_id, application_user_log_in_token_device_id.as_str()
            ).await {
                Ok(json_refresh_web_token) => {
                    if let Some(json_refresh_web_token_) = json_refresh_web_token {
                        json_refresh_web_token_registry.push(json_refresh_web_token_);
                    }
                }
                Err(mut error) => {
                    error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));
    
                    return Err(error);
                }
            }
        }

        if !json_refresh_web_token_registry.is_empty() {
            return Ok(Some(json_refresh_web_token_registry));
        }

        return Ok(None);
    }
}