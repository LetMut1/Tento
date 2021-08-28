use crate::domain_layer::entity::entity::json_access_web_token::_component::payload::_component::id::Id as JsonAccessWebTokenId;
use crate::domain_layer::repository::data_provider::_in_context_for::domain_layer::entity::entity::json_access_web_token_black_list::_new_for_context::_in_context_for::_resource::redis::_new_for_context::base_trait::BaseTrait as DataProviderJsonAccessWebTokenBlackListRedisTrait;
use crate::infrastructure_layer::error::base_error::base_error::BaseError;
use crate::infrastructure_layer::service::_in_context_for::infrastructure_layer::repository::_new_for_context::resource_storage_key_resolver::redis_storage_key_resolver::RedisStorageKeyResolver;
use redis::Commands;
use redis::Connection;

pub struct Base;

impl DataProviderJsonAccessWebTokenBlackListRedisTrait for Base {
    fn is_exist_by_json_access_token_id<'outer_a>(
        connection: &'outer_a mut Connection, json_access_web_token_id: &'outer_a JsonAccessWebTokenId
    ) -> Result<bool, BaseError> {
        return Ok(
            connection.exists::<String, bool>(
                RedisStorageKeyResolver::get_repository_json_access_web_token_bkack_list_first(json_access_web_token_id)
            )?
        );
    }
}