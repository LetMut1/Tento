use crate::domain_layer::repository::data_provider::_in_context_for::domain_layer::entity::entity::json_access_web_token_black_list::_new_for_context::_in_context_for::_resource::redis::_new_for_context::base_trait::BaseTrait as DataProviderJsonAccessWebTokenBlackListRedisTrait;
use crate::infrastructure_layer::error::base_error::base_error::BaseError;
use crate::infrastructure_layer::service::_in_context_for::infrastructure_layer::repository::_new_for_context::_in_context_for::_resource::redis::_new_for_context::storage_key_resolver::StorageKeyResolver;
use redis::Commands;
use redis::Connection;

pub struct Base;

impl DataProviderJsonAccessWebTokenBlackListRedisTrait for Base {
    type Error = BaseError;

    fn is_exist_by_json_access_token_id<'outer_a>(
        connection: &'outer_a mut Connection,
        json_access_web_token_id: &'outer_a str
    ) -> Result<bool, Self::Error> {
        return Ok(
            connection.exists::<String, bool>(
                StorageKeyResolver::get_repository_json_access_web_token_bkack_list_first(json_access_web_token_id)
            )?
        );
    }
}