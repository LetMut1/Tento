use crate::entity::entity::application_user::_core::id::Id;
use crate::error::base_error::base_error::BaseError;
use crate::utility::_in_context_for::repository::_new_for_context::resource_storage_key_resolver::redis_storage_key_resolver::RedisStorageKeyResolver;
use crate::utility::date_time_expiration_resolver::DateTimeExpirationResolver;
use redis::Commands;
use redis::Connection;

pub struct ProcessingDeviceIdStorage;

impl ProcessingDeviceIdStorage {
    const SEPARATOR: &'static str = ":";

    pub fn create<'outer_a>(
        connection: &'outer_a mut Connection, 
        application_user_id: &'outer_a Id,
        application_user_log_in_token_device_id_registry: Vec<String>
    ) -> Result<(), BaseError> {
        connection.set_ex::<String, String, ()>(
            RedisStorageKeyResolver::get_utility_json_refresh_web_token_first(application_user_id), 
            application_user_log_in_token_device_id_registry.join(Self::SEPARATOR),
            (DateTimeExpirationResolver::QUANTITY_OF_MINUTES_JSON_REFRESH_WEB_TOKEN_FIRST * 60) as usize
        )?;

        return Ok(());
    }

    pub fn update<'outer_a>(
        connection: &'outer_a mut Connection, 
        application_user_id: &'outer_a Id,
        application_user_log_in_token_device_id_registry: Vec<String>
    ) -> Result<(), BaseError> {
        Self::create(connection, application_user_id, application_user_log_in_token_device_id_registry)?;

        return Ok(());
    }

    pub fn delete<'outer_a>(
        connection: &'outer_a mut Connection, application_user_id: &'outer_a Id,
    ) -> Result<(), BaseError> {
        connection.del::<String, ()>(
            RedisStorageKeyResolver::get_utility_json_refresh_web_token_first(application_user_id)
        )?;
        
        return Ok(());
    }

    pub fn update_expiration_time<'outer_a>(
        connection: &'outer_a mut Connection, application_user_id: &'outer_a Id
    ) -> Result<(), BaseError> {
        connection.expire::<String, ()>(
            RedisStorageKeyResolver::get_utility_json_refresh_web_token_first(application_user_id),
            (DateTimeExpirationResolver::QUANTITY_OF_MINUTES_JSON_REFRESH_WEB_TOKEN_FIRST * 60) as usize
        )?;

        return Ok(());
    }

    pub fn get<'outer_a>(
        connection: &'outer_a mut Connection, application_user_id: &'outer_a Id
    ) -> Result<Option<Vec<String>>, BaseError> {
        if let Some(application_user_log_in_token_device_id_sequence) = connection.get::<String, Option<String>>(
            RedisStorageKeyResolver::get_utility_json_refresh_web_token_first(application_user_id)
        )?
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