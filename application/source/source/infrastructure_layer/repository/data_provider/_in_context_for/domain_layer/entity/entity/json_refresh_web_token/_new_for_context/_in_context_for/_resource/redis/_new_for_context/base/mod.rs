use crate::domain_layer::entity::entity::application_user_log_in_token::_component::device_id::DeviceId as ApplicationUserLogInTokenDeviceId;
use crate::domain_layer::entity::entity::application_user::_component::id::Id as ApplicationUserId;
use crate::domain_layer::entity::entity::json_refresh_web_token::json_refresh_web_token::JsonRefreshWebToken;
use crate::domain_layer::repository::data_provider::_in_context_for::domain_layer::entity::entity::json_refresh_web_token::_new_for_context::_in_context_for::_resource::redis::_new_for_context::base_trait::BaseTrait as DataProviderJsonRefreshWebTokenRedisTrait;
use crate::domain_layer::service::factory::_in_context_for::domain_layer::entity::entity::json_refresh_web_token::_new_for_context::base::Base as JsonRefreshWebTokenFactory;
use crate::infrastructure_layer::data_transfer_object::_in_context_for::infrastructure_layer::repository::state_manager::_in_context_for::domain_layer::entity::entity::json_refresh_web_token::_new_for_context::_in_context_for::_resource::redis::_new_for_context::base::_new_for_context::common::Common;
use crate::infrastructure_layer::error::base_error::base_error::BaseError;
use crate::infrastructure_layer::service::_in_context_for::infrastructure_layer::repository::_new_for_context::resource_storage_key_resolver::redis_storage_key_resolver::RedisStorageKeyResolver;
use redis::Commands;
use redis::Connection;

pub struct Base;

impl DataProviderJsonRefreshWebTokenRedisTrait for Base {
    fn get_by_application_user_id_and_application_user_log_in_token_device_id<'outer_a>(
        connection: &'outer_a mut Connection, 
        application_user_id: &'outer_a ApplicationUserId, 
        application_user_log_in_token_device_id: &'outer_a ApplicationUserLogInTokenDeviceId,
    ) -> Result<Option<JsonRefreshWebToken<'static>>, BaseError> {
        match connection.get::<String, Option<String>>(
            RedisStorageKeyResolver::get_repository_json_refresh_web_token_first(application_user_id, application_user_log_in_token_device_id)
        )?
        {
            Some(json_encoded_common) => {
                return Ok(Some(JsonRefreshWebTokenFactory::new_from_common(serde_json::from_str::<'_, Common<'_>>(json_encoded_common.as_str())?)?));
            },
            None => {
                return Ok(None);
            }
        }
    }

    fn get_by_application_user_id_and_application_user_log_in_token_device_id_registry<'outer_a>(
        connection: &'outer_a mut Connection, 
        application_user_id: &'outer_a ApplicationUserId, 
        application_user_log_in_token_device_id_registry: Vec<String>
    ) -> Result<Option<Vec<JsonRefreshWebToken<'static>>>, BaseError> {
        let mut json_refresh_web_token_registry: Vec<JsonRefreshWebToken<'_>> = Vec::new();

        for application_user_log_in_token_device_id in application_user_log_in_token_device_id_registry.into_iter() {
            if let Some(json_refresh_web_token) = Self::get_by_application_user_id_and_application_user_log_in_token_device_id(
                connection, application_user_id, &ApplicationUserLogInTokenDeviceId::new_from_string(application_user_log_in_token_device_id)?
            )?
            {
                json_refresh_web_token_registry.push(json_refresh_web_token);
            }
        }

        if !json_refresh_web_token_registry.is_empty() {
            return Ok(Some(json_refresh_web_token_registry));
        }

        return Ok(None);
    }
}