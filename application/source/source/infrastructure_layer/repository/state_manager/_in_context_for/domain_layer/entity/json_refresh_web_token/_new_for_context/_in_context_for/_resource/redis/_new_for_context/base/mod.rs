use crate::domain_layer::entity::json_refresh_web_token::JsonRefreshWebToken;
use crate::domain_layer::repository::state_manager::_in_context_for::domain_layer::entity::json_refresh_web_token::_new_for_context::_in_context_for::_resource::redis::_new_for_context::base_trait::BaseTrait as JsonRefreshWebTokenStateManagerRedisTrait;
use crate::infrastructure_layer::data_transfer_object::_in_context_for::infrastructure_layer::repository::state_manager::_in_context_for::domain_layer::entity::json_refresh_web_token::_new_for_context::_in_context_for::_resource::redis::_new_for_context::base::_new_for_context::common::Common;
use crate::infrastructure_layer::error::base_error::base_error::BaseError;
use crate::infrastructure_layer::service::_in_context_for::infrastructure_layer::repository::_new_for_context::_in_context_for::_resource::redis::_new_for_context::storage_key_resolver::StorageKeyResolver;
use redis::Commands;
use redis::Connection;

#[doc = "Should only be used in use crate::infrastructure_layer::service::_in_context_for::domain_layer::entity::json_refresh_web_token::_new_for_context::repository_proxy::RepositoryProxy."]
pub struct Base;

impl JsonRefreshWebTokenStateManagerRedisTrait for Base {
    type Error = BaseError;

    fn create<'outer_a>(
        connection: &'outer_a mut Connection,
        json_refresh_web_token: &'outer_a JsonRefreshWebToken<'_>
    ) -> Result<(), Self::Error> {
        connection.set_ex::<String, String, ()>(
            StorageKeyResolver::get_repository_json_refresh_web_token_first(
                json_refresh_web_token.get_application_user_id(), json_refresh_web_token.get_application_user_log_in_token_device_id()
            ), 
            serde_json::to_string(&Common::new(json_refresh_web_token))?,
            (JsonRefreshWebToken::QUANTITY_OF_MINUTES_FOR_EXPIRATION as usize) * (60 as usize)
        )?;

        return Ok(());
    }

    fn update<'outer_a>(
        connection: &'outer_a mut Connection,
        json_refresh_web_token: &'outer_a JsonRefreshWebToken<'_>
    ) -> Result<(), Self::Error> {
        Self::create(connection, json_refresh_web_token)?;

        return Ok(());
    }


    fn delete<'outer_a>(
        connection: &'outer_a mut Connection,
        json_refresh_web_token: &'outer_a JsonRefreshWebToken<'_>
    ) -> Result<(), Self::Error> {
        connection.del::<String, ()>(
            StorageKeyResolver::get_repository_json_refresh_web_token_first(
                json_refresh_web_token.get_application_user_id(), json_refresh_web_token.get_application_user_log_in_token_device_id()
            )
        )?;
        
        return Ok(());
    }
}