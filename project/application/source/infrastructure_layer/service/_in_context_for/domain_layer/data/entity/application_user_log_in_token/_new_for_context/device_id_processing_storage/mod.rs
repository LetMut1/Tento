use crate::domain_layer::data::entity::json_refresh_web_token::JsonRefreshWebToken;
use crate::infrastructure_layer::data_transfer_object::error_auditor::_component::base_error::_component::run_time_error::_component::resource_error::resource_error::ResourceError;
use crate::infrastructure_layer::data_transfer_object::error_auditor::_component::base_error::_component::run_time_error::run_time_error::RunTimeError;
use crate::infrastructure_layer::data_transfer_object::error_auditor::_component::base_error::base_error::BaseError;
use crate::infrastructure_layer::data_transfer_object::error_auditor::_component::simple_backtrace::_component::backtrace_part::BacktracePart;
use crate::infrastructure_layer::data_transfer_object::error_auditor::error_auditor::ErrorAuditor;
use crate::infrastructure_layer::service::_in_context_for::infrastructure_layer::repository::_new_for_context::_in_context_for::_resource::redis::_new_for_context::storage_key_resolver::StorageKeyResolver;
use redis::aio::Connection;
use redis::AsyncCommands;

pub struct DeviceIdProcessingStorage;       // УДАЛИТЬ, КОГДА СОХРАНение токенов будет в Постгресе

impl DeviceIdProcessingStorage {
    const SEPARATOR: &'static str = ":";

    pub async fn create<'a>(
        connection: &'a mut Connection, 
        application_user_id: i64,
        application_user_log_in_token_device_id_registry: Vec<String>
    ) -> Result<(), ErrorAuditor> {
        if let Err(error) = connection.set_ex::<String, String, ()>(
            StorageKeyResolver::get_6(application_user_id), 
            application_user_log_in_token_device_id_registry.join(Self::SEPARATOR),
            (JsonRefreshWebToken::QUANTITY_OF_MINUTES_FOR_EXPIRATION as usize) * (60 as usize)
        ).await {
            return Err(
                ErrorAuditor::new(
                    BaseError::RunTimeError { run_time_error: RunTimeError::ResourceError { resource_error: ResourceError::RedisError { redis_error: error } } },
                    BacktracePart::new(line!(), file!(), None)
                )
            );
        }

        return Ok(());
    }

    pub async fn update<'a>(
        connection: &'a mut Connection, 
        application_user_id: i64,
        application_user_log_in_token_device_id_registry: Vec<String>
    ) -> Result<(), ErrorAuditor> {
        if let Err(mut error) = Self::create(connection, application_user_id, application_user_log_in_token_device_id_registry).await {
            error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));
    
            return Err(error);
        }

        return Ok(());
    }

    pub async fn delete<'a>(
        connection: &'a mut Connection,
        application_user_id: i64,
    ) -> Result<(), ErrorAuditor> {
        if let Err(error) = connection.del::<String, ()>(
            StorageKeyResolver::get_6(application_user_id)
        ).await {
            return Err(
                ErrorAuditor::new(
                    BaseError::RunTimeError { run_time_error: RunTimeError::ResourceError { resource_error: ResourceError::RedisError { redis_error: error } } },
                    BacktracePart::new(line!(), file!(), None)
                )
            );
        }
        
        return Ok(());
    }

    pub async fn update_expiration_time<'a>(
        connection: &'a mut Connection,
        application_user_id: i64
    ) -> Result<(), ErrorAuditor> {
        if let Err(error) = connection.expire::<String, ()>(
            StorageKeyResolver::get_6(application_user_id),
            (JsonRefreshWebToken::QUANTITY_OF_MINUTES_FOR_EXPIRATION as usize) * (60 as usize)
        ).await {
            return Err(
                ErrorAuditor::new(
                    BaseError::RunTimeError { run_time_error: RunTimeError::ResourceError { resource_error: ResourceError::RedisError { redis_error: error } } },
                    BacktracePart::new(line!(), file!(), None)
                )
            );
        }

        return Ok(());
    }

    pub async fn get<'a>(
        connection: &'a mut Connection,
        application_user_id: i64
    ) -> Result<Option<Vec<String>>, ErrorAuditor> {
        match connection.get::<String, Option<String>>(
            StorageKeyResolver::get_6(application_user_id)
        ).await {
            Ok(application_user_log_in_token_device_id_sequence) => {
                if let Some(application_user_log_in_token_device_id_sequence_) = application_user_log_in_token_device_id_sequence {
                    let mut application_user_log_in_token_device_id_registry: Vec<String> = vec![];
        
                    '_a: for application_user_log_in_token_device_id in application_user_log_in_token_device_id_sequence_.split::<'_, &'_ str>(Self::SEPARATOR) {
                        application_user_log_in_token_device_id_registry.push(application_user_log_in_token_device_id.to_string());
                    }
        
                    return Ok(Some(application_user_log_in_token_device_id_registry));
                }
                
                return Ok(None);
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
}