use crate::domain_layer::entity::entity::json_refresh_web_token::json_refresh_web_token::JsonRefreshWebToken;
use crate::domain_layer::repository::state_manager::_in_context_for::domain_layer::entity::entity::json_refresh_web_token::_new_for_context::_in_context_for::_resource::redis::_new_for_context::base_trait::BaseTrait as StateManagerJsonRefreshWebTokenRedisTrait;
use crate::infrastructure_layer::data_transfer_object::_in_context_for::infrastructure_layer::repository::_in_context_for::domain_layer::entity::entity::json_refresh_web_token::_new_for_context::_in_context_for::_resource::redis::_new_for_context::base_repository::_new_for_context::common::Common;
use crate::infrastructure_layer::error::base_error::base_error::BaseError;
use crate::infrastructure_layer::service::_in_context_for::infrastructure_layer::repository::_new_for_context::resource_storage_key_resolver::redis_storage_key_resolver::RedisStorageKeyResolver;
use crate::infrastructure_layer::service::date_time_expiration_storage::DateTimeExpirationStorage;
use redis::Commands;
use redis::Connection;

#[doc = "Should only be used in use crate::infrastructure_layer::service::_in_context_for::domain_layer::entity::entity::json_refresh_web_token::_new_for_context::repository_proxy::RepositoryProxy."]
pub struct Base;

impl StateManagerJsonRefreshWebTokenRedisTrait for Base {
    fn create<'outer_a>(
        connection: &'outer_a mut Connection, json_refresh_web_token: &'outer_a JsonRefreshWebToken<'_>
    ) -> Result<(), BaseError> {
        connection.set_ex::<String, String, ()>(
            RedisStorageKeyResolver::get_repository_json_refresh_web_token_first(
                json_refresh_web_token.get_application_user_id(), json_refresh_web_token.get_application_user_log_in_token_device_id()
            ), 
            serde_json::to_string(&Common::new(json_refresh_web_token))?,
            (DateTimeExpirationStorage::QUANTITY_OF_MINUTES_JSON_REFRESH_WEB_TOKEN_FIRST as usize) * (60 as usize)
        )?;

        return Ok(());
    }

    fn update<'outer_a>(
        connection: &'outer_a mut Connection, json_refresh_web_token: &'outer_a JsonRefreshWebToken<'_>
    ) -> Result<(), BaseError> {
        Self::create(connection, json_refresh_web_token)?;

        return Ok(());
    }


    fn delete<'outer_a>(
        connection: &'outer_a mut Connection, json_refresh_web_token: &'outer_a JsonRefreshWebToken<'_>
    ) -> Result<(), BaseError> {
        connection.del::<String, ()>(
            RedisStorageKeyResolver::get_repository_json_refresh_web_token_first(
                json_refresh_web_token.get_application_user_id(), json_refresh_web_token.get_application_user_log_in_token_device_id()
            )
        )?;
        
        return Ok(());
    }
}