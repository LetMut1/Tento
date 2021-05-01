use crate::entity::core::uuid_v4::UuidV4;
use crate::entity::entity::json_access_web_token_black_list::json_access_web_token_black_list::JsonAccessWebTokenBlackList;
use crate::error::main_error_kind::core::resource_error_kind::resource_error_kind::ResourceErrorKind;
use crate::utility::_in_context_for::entity::entity::json_access_web_token::_new_for_context::date_expiration_creator::DateExpirationCreator;
use crate::utility::_in_context_for::repository::_new_for_context::resource_storage_key_resolver::redis_storage_key_resolver::RedisStorageKeyResolver;
use crate::utility::resource_connection::redis::connection_manager::ConnectionManager;
use redis::Commands;

pub struct BaseRepository;

impl<'outer, 'vague> BaseRepository {
    pub fn create(
        connection_manager: &'outer mut ConnectionManager, json_access_web_token_black_list: &'outer JsonAccessWebTokenBlackList<'outer>
    ) -> Result<(), ResourceErrorKind> {
        return Ok(
            connection_manager.get_connection().set_ex::<String, u8, ()>(
                RedisStorageKeyResolver::get_repository_json_access_web_token_bkack_list_first(
                    json_access_web_token_black_list.get_json_access_web_token_id()
                ), 
                1,
                (DateExpirationCreator::QUANTITY_OF_MINUTES * 60) as usize
            )?
        );
    }

    pub fn is_exist_by_json_access_token_id(
        connection_manager: &'outer mut ConnectionManager, json_access_web_token_id: &'outer UuidV4
    ) -> Result<bool, ResourceErrorKind> {
        return Ok(
            connection_manager.get_connection().exists::<String, bool>(
                RedisStorageKeyResolver::get_repository_json_access_web_token_bkack_list_first(json_access_web_token_id)
            )?
        );
    }
}