use crate::infrastructure_layer::error::base_error::base_error::BaseError;
use crate::infrastructure_layer::service::_in_context_for::infrastructure_layer::repository::_new_for_context::_in_context_for::_resource::redis::_new_for_context::storage_key_resolver::StorageKeyResolver;
use redis_ref::aio::Connection;
use redis_ref::AsyncCommands;

pub struct Base;

impl Base {
    pub async fn is_exist_by_json_access_token_id<'a>(
        connection: &'a mut Connection,
        json_access_web_token_id: &'a str
    ) -> Result<bool, BaseError> {
        let result = connection.exists::<String, bool>(StorageKeyResolver::get_4(json_access_web_token_id)).await?;

        return Ok(result);
    }
}