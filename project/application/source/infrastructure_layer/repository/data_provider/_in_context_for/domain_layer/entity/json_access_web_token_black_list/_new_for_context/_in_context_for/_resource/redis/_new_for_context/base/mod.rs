use crate::infrastructure_layer::error::error_aggregator::error_aggregator::ErrorAggregator;
use crate::infrastructure_layer::service::_in_context_for::infrastructure_layer::repository::_new_for_context::_in_context_for::_resource::redis::_new_for_context::storage_key_resolver::StorageKeyResolver;
use redis::aio::Connection;
use redis::AsyncCommands;

pub struct Base;

impl Base {
    pub async fn is_exist_by_json_access_token_id<'a>(
        connection: &'a mut Connection,
        json_access_web_token_id: &'a str
    ) -> Result<bool, ErrorAggregator> {
        let result = connection.exists::<String, bool>(StorageKeyResolver::get_4(json_access_web_token_id)).await?;

        return Ok(result);
    }
}