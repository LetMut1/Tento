use crate::domain_layer::entity::entity::application_user::_component::id::Id;
use crate::infrastructure_layer::error::base_error::base_error::BaseError;
use crate::infrastructure_layer::service::_in_context_for::infrastructure_layer::repository::_new_for_context::_in_context_for::_resource::redis::_new_for_context::storage_key_resolver::StorageKeyResolver;
use crate::infrastructure_layer::service::date_time_expiration_storage::DateTimeExpirationStorage;
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
            StorageKeyResolver::get_service_json_refresh_web_token_first(application_user_id), 
            application_user_log_in_token_device_id_registry.join(Self::SEPARATOR),
            (DateTimeExpirationStorage::QUANTITY_OF_MINUTES_JSON_REFRESH_WEB_TOKEN_FIRST as usize) * (60 as usize)
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
            StorageKeyResolver::get_service_json_refresh_web_token_first(application_user_id)
        )?;
        
        return Ok(());
    }

    pub fn update_expiration_time<'outer_a>(
        connection: &'outer_a mut Connection, application_user_id: &'outer_a Id
    ) -> Result<(), BaseError> {
        connection.expire::<String, ()>(
            StorageKeyResolver::get_service_json_refresh_web_token_first(application_user_id),
            (DateTimeExpirationStorage::QUANTITY_OF_MINUTES_JSON_REFRESH_WEB_TOKEN_FIRST as usize) * (60 as usize)
        )?;

        return Ok(());
    }

    pub fn get<'outer_a>(
        connection: &'outer_a mut Connection, application_user_id: &'outer_a Id
    ) -> Result<Option<Vec<String>>, BaseError> {
        if let Some(application_user_log_in_token_device_id_sequence) = connection.get::<String, Option<String>>(
            StorageKeyResolver::get_service_json_refresh_web_token_first(application_user_id)
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