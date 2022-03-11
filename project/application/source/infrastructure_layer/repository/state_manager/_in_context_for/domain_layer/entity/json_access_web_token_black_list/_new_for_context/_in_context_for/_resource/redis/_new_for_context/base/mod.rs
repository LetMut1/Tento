use crate::domain_layer::entity::json_access_web_token_black_list::JsonAccessWebTokenBlackList;
use crate::domain_layer::entity::json_access_web_token::json_access_web_token::JsonAccessWebToken;
use crate::infrastructure_layer::error::base_error::base_error::BaseError;
use crate::infrastructure_layer::service::_in_context_for::infrastructure_layer::repository::_new_for_context::_in_context_for::_resource::redis::_new_for_context::storage_key_resolver::StorageKeyResolver;
use redis::aio::Connection;
use redis::AsyncCommands;

pub struct Base;

impl Base {
    pub async fn create<'a>(
        connection: &'a mut Connection,
        json_access_web_token_black_list: &'a JsonAccessWebTokenBlackList<'_>
    ) -> Result<(), BaseError> {
        connection.set_ex::<String, u8, ()>(
            StorageKeyResolver::get_4(
                json_access_web_token_black_list.get_json_access_web_token_id()
            ), 
            1,
            (JsonAccessWebToken::QUANTITY_OF_MINUTES_FOR_EXPIRATION as usize) * (60 as usize)
        ).await?;

        return Ok(());
    }
}