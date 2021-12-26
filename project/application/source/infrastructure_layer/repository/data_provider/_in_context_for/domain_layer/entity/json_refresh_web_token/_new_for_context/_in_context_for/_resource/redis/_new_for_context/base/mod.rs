use crate::domain_layer::entity::json_refresh_web_token::JsonRefreshWebToken;
use crate::domain_layer::repository::data_provider::_in_context_for::domain_layer::entity::json_refresh_web_token::_new_for_context::_in_context_for::_resource::redis::_new_for_context::base_trait::BaseTrait as JsonRefreshWebTokenDataProviderRedisTrait;
use crate::infrastructure_layer::data_transfer_object::_in_context_for::infrastructure_layer::repository::state_manager::_in_context_for::domain_layer::entity::json_refresh_web_token::_new_for_context::_in_context_for::_resource::redis::_new_for_context::base::_new_for_context::common::Common;
use crate::infrastructure_layer::error::base_error::base_error::BaseError;
use crate::infrastructure_layer::service::_in_context_for::infrastructure_layer::repository::_new_for_context::_in_context_for::_resource::redis::_new_for_context::storage_key_resolver::StorageKeyResolver;
use redis::Commands;
use redis::Connection;
use std::borrow::Cow;

pub struct Base;

impl JsonRefreshWebTokenDataProviderRedisTrait for Base {
    type Error = BaseError;

    fn find_by_application_user_id_and_application_user_log_in_token_device_id<'a>(
        connection: &'a mut Connection, 
        application_user_id: &'a i64, 
        application_user_log_in_token_device_id: &'a str,
    ) -> Result<Option<JsonRefreshWebToken<'static>>, Self::Error> {
        match connection.get::<String, Option<String>>(
            StorageKeyResolver::get_5(application_user_id, application_user_log_in_token_device_id)
        )? {
            Some(json_encoded_common) => {
                let common: Common<'static> = serde_json::from_str::<'_, Common<'static>>(json_encoded_common.as_str())?;

                let (
                    json_access_web_token_id,
                    application_user_id,
                    application_user_log_in_token_device_id,
                    json_refresh_web_token_obfuscation_value
                ) : (
                    Cow<'static, str>,
                    Cow<'static, i64>,
                    Cow<'static, str>,
                    Cow<'static, str>
                ) = common.into_inner();
        
                let json_refresh_web_token:JsonRefreshWebToken<'static> = JsonRefreshWebToken::new(
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

    fn find_by_application_user_id_and_application_user_log_in_token_device_id_registry<'a>(
        connection: &'a mut Connection, 
        application_user_id: &'a i64, 
        application_user_log_in_token_device_id_registry: Vec<String>
    ) -> Result<Option<Vec<JsonRefreshWebToken<'static>>>, Self::Error> {
        let mut json_refresh_web_token_registry: Vec<JsonRefreshWebToken<'_>> = Vec::new();

        for application_user_log_in_token_device_id in application_user_log_in_token_device_id_registry.into_iter() {
            if let Some(json_refresh_web_token) = Self::find_by_application_user_id_and_application_user_log_in_token_device_id(
                connection, application_user_id, application_user_log_in_token_device_id.as_str()
            )? {
                json_refresh_web_token_registry.push(json_refresh_web_token);
            }
        }

        if !json_refresh_web_token_registry.is_empty() {
            return Ok(Some(json_refresh_web_token_registry));
        }

        return Ok(None);
    }
}