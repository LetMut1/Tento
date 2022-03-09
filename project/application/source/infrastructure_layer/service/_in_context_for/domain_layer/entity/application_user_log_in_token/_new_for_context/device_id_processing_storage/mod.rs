use crate::domain_layer::entity::json_refresh_web_token::JsonRefreshWebToken;
use crate::infrastructure_layer::error::base_error::base_error::BaseError;
use crate::infrastructure_layer::service::_in_context_for::infrastructure_layer::repository::_new_for_context::_in_context_for::_resource::redis::_new_for_context::storage_key_resolver::StorageKeyResolver;
use redis_ref::aio::Connection;
use redis_ref::AsyncCommands;

pub struct DeviceIdProcessingStorage;

impl DeviceIdProcessingStorage {
    const SEPARATOR: &'static str = ":";

    pub async fn create<'a>(
        connection: &'a mut Connection, 
        application_user_id: &'a i64,
        application_user_log_in_token_device_id_registry: Vec<String>
    ) -> Result<(), BaseError> {
        connection.set_ex::<String, String, ()>(
            StorageKeyResolver::get_6(application_user_id), 
            application_user_log_in_token_device_id_registry.join(Self::SEPARATOR),
            (JsonRefreshWebToken::QUANTITY_OF_MINUTES_FOR_EXPIRATION as usize) * (60 as usize)
        ).await?;

        return Ok(());
    }

    pub async fn update<'a>(
        connection: &'a mut Connection, 
        application_user_id: &'a i64,
        application_user_log_in_token_device_id_registry: Vec<String>
    ) -> Result<(), BaseError> {
        Self::create(connection, application_user_id, application_user_log_in_token_device_id_registry).await?;

        return Ok(());
    }

    pub async fn delete<'a>(
        connection: &'a mut Connection,
        application_user_id: &'a i64,
    ) -> Result<(), BaseError> {
        connection.del::<String, ()>(
            StorageKeyResolver::get_6(application_user_id)
        ).await?;
        
        return Ok(());
    }

    pub async fn update_expiration_time<'a>(
        connection: &'a mut Connection,
        application_user_id: &'a i64
    ) -> Result<(), BaseError> {
        connection.expire::<String, ()>(
            StorageKeyResolver::get_6(application_user_id),
            (JsonRefreshWebToken::QUANTITY_OF_MINUTES_FOR_EXPIRATION as usize) * (60 as usize)
        ).await?;

        return Ok(());
    }

    pub async fn get<'a>(
        connection: &'a mut Connection,
        application_user_id: &'a i64
    ) -> Result<Option<Vec<String>>, BaseError> {
        if let Some(application_user_log_in_token_device_id_sequence) = connection.get::<String, Option<String>>(
            StorageKeyResolver::get_6(application_user_id)
        ).await?
        {
            let mut application_user_log_in_token_device_id_registry: Vec<String> = Vec::new();

            for application_user_log_in_token_device_id in application_user_log_in_token_device_id_sequence.split::<'_, &'_ str>(Self::SEPARATOR) {
                application_user_log_in_token_device_id_registry.push(application_user_log_in_token_device_id.to_string());
            }

            return Ok(Some(application_user_log_in_token_device_id_registry));
        }
        
        return Ok(None);
    }
}