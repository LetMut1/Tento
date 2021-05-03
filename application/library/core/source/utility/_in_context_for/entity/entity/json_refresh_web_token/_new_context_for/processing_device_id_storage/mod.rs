use crate::entity::entity::application_user::core::id::Id;
use crate::error::main_error_kind::core::resource_error_kind::resource_error_kind::ResourceErrorKind;
use crate::utility::_in_context_for::repository::_new_for_context::resource_storage_key_resolver::redis_storage_key_resolver::RedisStorageKeyResolver;
use crate::utility::date_time_expiration_resolver::DateTimeExpirationResolver;
use crate::utility::resource_connection::redis::connection_manager::ConnectionManager;
use redis::Commands;

pub struct ProcessingDeviceIdStorage;

impl<'outer_a> ProcessingDeviceIdStorage {
    const SEPARATOR: &'static str = ":";

    pub fn create(
        connection_manager: &'outer_a mut ConnectionManager, 
        application_user_id: &'outer_a Id,
        application_user_log_in_token_device_id_registry: Vec<String>
    ) -> Result<(), ResourceErrorKind> {
        connection_manager.get_connection().set_ex::<String, String, ()>(
            RedisStorageKeyResolver::get_utility_json_refresh_web_token_first(application_user_id), 
            application_user_log_in_token_device_id_registry.join(Self::SEPARATOR),
            (DateTimeExpirationResolver::QUANTITY_OF_MINUTES_JSON_REFRESH_WEB_TOKEN_FIRST * 60) as usize
        )?;

        return Ok(());
    }

    pub fn update(
        connection_manager: &'outer_a mut ConnectionManager, 
        application_user_id: &'outer_a Id,
        application_user_log_in_token_device_id_registry: Vec<String>
    ) -> Result<(), ResourceErrorKind> {
        Self::create(connection_manager, application_user_id, application_user_log_in_token_device_id_registry)?;

        return Ok(());
    }

    pub fn delete(
        connection_manager: &'outer_a mut ConnectionManager, application_user_id: &'outer_a Id,
    ) -> Result<(), ResourceErrorKind> {
        connection_manager.get_connection().del::<String, ()>(
            RedisStorageKeyResolver::get_utility_json_refresh_web_token_first(application_user_id)
        )?;
        
        return Ok(());
    }

    pub fn update_expiration_time(
        connection_manager: &'outer_a mut ConnectionManager, application_user_id: &'outer_a Id
    ) -> Result<(), ResourceErrorKind> {
        connection_manager.get_connection().expire::<String, ()>(
            RedisStorageKeyResolver::get_utility_json_refresh_web_token_first(application_user_id),
            (DateTimeExpirationResolver::QUANTITY_OF_MINUTES_JSON_REFRESH_WEB_TOKEN_FIRST * 60) as usize
        )?;

        return Ok(());
    }

    pub fn get(
        connection_manager: &'outer_a mut ConnectionManager, application_user_id: &'outer_a Id
    ) -> Result<Option<Vec<String>>, ResourceErrorKind> {
        if let Some(application_user_log_in_token_device_id_sequence) = connection_manager.get_connection().get::<String, Option<String>>(
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