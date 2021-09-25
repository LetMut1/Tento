use crate::domain_layer::entity::json_access_web_token_black_list::JsonAccessWebTokenBlackList;
use crate::domain_layer::entity::json_access_web_token::json_access_web_token::JsonAccessWebToken;
use crate::domain_layer::repository::state_manager::_in_context_for::domain_layer::entity::json_access_web_token_black_list::_new_for_context::_in_context_for::_resource::redis::_new_for_context::base_trait::BaseTrait as JsonAccessWebTokenBlackListStateManagerRedisTrait;
use crate::infrastructure_layer::error::base_error::base_error::BaseError;
use crate::infrastructure_layer::service::_in_context_for::infrastructure_layer::repository::_new_for_context::_in_context_for::_resource::redis::_new_for_context::storage_key_resolver::StorageKeyResolver;
use redis::Commands;
use redis::Connection;

pub struct Base;

impl JsonAccessWebTokenBlackListStateManagerRedisTrait for Base {
    type Error = BaseError;

    fn create<'outer_a>(
        connection: &'outer_a mut Connection,
        json_access_web_token_black_list: &'outer_a JsonAccessWebTokenBlackList<'_>
    ) -> Result<(), Self::Error> {
        connection.set_ex::<String, u8, ()>(
            StorageKeyResolver::get_repository_json_access_web_token_bkack_list_first(
                json_access_web_token_black_list.get_json_access_web_token_id()
            ), 
            1,
            (JsonAccessWebToken::QUANTITY_OF_MINUTES_FOR_EXPIRATION as usize) * (60 as usize)
        )?;

        return Ok(());
    }
}