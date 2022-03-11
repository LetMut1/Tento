use crate::domain_layer::entity::json_refresh_web_token::JsonRefreshWebToken;
use crate::infrastructure_layer::data_transfer_object::_in_context_for::infrastructure_layer::repository::state_manager::_in_context_for::domain_layer::entity::json_refresh_web_token::_new_for_context::_in_context_for::_resource::redis::_new_for_context::base::_new_for_context::common::Common;
use crate::infrastructure_layer::error::base_error::base_error::BaseError;
use crate::infrastructure_layer::service::_in_context_for::infrastructure_layer::repository::_new_for_context::_in_context_for::_resource::redis::_new_for_context::storage_key_resolver::StorageKeyResolver;
use redis::aio::Connection;
use redis::AsyncCommands;

pub struct Base;

impl Base {
    pub async fn find_by_application_user_id_and_application_user_log_in_token_device_id<'a>(
        connection: &'a mut Connection, 
        application_user_id: &'a i64, 
        application_user_log_in_token_device_id: &'a str,
    ) -> Result<Option<JsonRefreshWebToken<'static>>, BaseError> {
        match connection.get::<String, Option<Vec<u8>>>(StorageKeyResolver::get_5(application_user_id, application_user_log_in_token_device_id)).await? {
            Some(data) => {
                let common = rmp_serde::from_read_ref::<'_, [u8], Common<'static>>(&data[..])?;

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
            },
            None => {
                return Ok(None);
            }
        }
    }

    pub async fn find_by_application_user_id_and_application_user_log_in_token_device_id_registry<'a>(
        connection: &'a mut Connection, 
        application_user_id: &'a i64, 
        application_user_log_in_token_device_id_registry: Vec<String>
    ) -> Result<Option<Vec<JsonRefreshWebToken<'static>>>, BaseError> {
        let mut json_refresh_web_token_registry: Vec<JsonRefreshWebToken<'_>> = Vec::new();

        for application_user_log_in_token_device_id in application_user_log_in_token_device_id_registry.into_iter() {
            if let Some(json_refresh_web_token) = Self::find_by_application_user_id_and_application_user_log_in_token_device_id(
                connection, application_user_id, application_user_log_in_token_device_id.as_str()
            ).await? {
                json_refresh_web_token_registry.push(json_refresh_web_token);
            }
        }

        if !json_refresh_web_token_registry.is_empty() {
            return Ok(Some(json_refresh_web_token_registry));
        }

        return Ok(None);
    }
}