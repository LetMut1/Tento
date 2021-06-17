use crate::entity::entity::json_access_web_token_black_list::json_access_web_token_black_list::JsonAccessWebTokenBlackList;
use crate::entity::entity::json_access_web_token::_core::payload::_core::id::Id as JsonAccessWebTokenId;
use crate::error::base_error::base_error::BaseError;
use crate::utility::_in_context_for::repository::_new_for_context::resource_storage_key_resolver::redis_storage_key_resolver::RedisStorageKeyResolver;
use crate::utility::date_time_expiration_resolver::DateTimeExpirationResolver;
use redis::Commands;
use redis::Connection;

pub struct BaseRepository;

impl BaseRepository {
    pub fn create<'outer_a>(
        connection: &'outer_a mut Connection, json_access_web_token_black_list: &'outer_a JsonAccessWebTokenBlackList<'_>
    ) -> Result<(), BaseError> {
        connection.set_ex::<String, u8, ()>(
            RedisStorageKeyResolver::get_repository_json_access_web_token_bkack_list_first(
                json_access_web_token_black_list.get_json_access_web_token_id()
            ), 
            1,
            (DateTimeExpirationResolver::QUANTITY_OF_MINUTES_JSON_ACCESS_WEB_TOKEN_FIRST * 60) as usize
        )?;

        return Ok(());
    }

    pub fn is_exist_by_json_access_token_id<'outer_a>(
        connection: &'outer_a mut Connection, json_access_web_token_id: &'outer_a JsonAccessWebTokenId
    ) -> Result<bool, BaseError> {
        return Ok(
            connection.exists::<String, bool>(
                RedisStorageKeyResolver::get_repository_json_access_web_token_bkack_list_first(json_access_web_token_id)
            )?
        );
    }
}