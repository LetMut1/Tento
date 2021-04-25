use crate::entity::entity::json_access_web_token_black_list::json_access_web_token_black_list::JsonAccessWebTokenBlackList;
use crate::entity::entity::json_access_web_token::json_access_web_token::JsonAccessWebToken;
use crate::error::main_error_kind::core::entity_error_kind::core::_in_context_for::entity::entity::json_refresh_web_token::_new_for_context::json_refresh_web_token_error_kind::JsonRefreshWebTokenErrorKind;
use crate::error::main_error_kind::core::entity_error_kind::entity_error_kind::EntityErrorKind;
use crate::error::main_error_kind::main_error_kind::MainErrorKind;
use crate::repository::_in_context_for::entity::entity::json_access_web_token_black_list::_new_for_context::postgresql::base_repository::BaseRepository as JsonAccessWebTokenBlackListRepository;
use crate::repository::_in_context_for::entity::entity::json_refresh_web_token::_new_for_context::postgresql::base_repository::BaseRepository as JsonRefreshWebTokenBaseRepository;
use crate::utility::resource_connection::postgresql::connection_manager::ConnectionManager as PostgresqlConnectionManager;
use crate::utility::resource_connection::redis::connection_manager::ConnectionManager as RedisConnectionManager;

pub struct Handler;

impl<'outer> Handler {
    pub fn handle(json_access_web_token: &'outer JsonAccessWebToken<'outer>) -> Result<(), MainErrorKind> {
        let mut postgresql_connection_manager: PostgresqlConnectionManager = PostgresqlConnectionManager::new();
        postgresql_connection_manager.establish_connection()?;

        if let Some(json_refresh_web_token) = JsonRefreshWebTokenBaseRepository::get_by_application_user_id_and_application_user_log_in_token_device_id(
            &postgresql_connection_manager, json_access_web_token.get_application_user_id(), json_access_web_token.get_application_user_log_in_token_device_id()
        )?
        {
            JsonRefreshWebTokenBaseRepository::delete(&postgresql_connection_manager, &json_refresh_web_token)?;  // TODO без транзакции, так как все будет на кеше (Удалить это сообщение, как только перепишу на Кеш)

            let mut redis_connection_manager: RedisConnectionManager = RedisConnectionManager::new();
            redis_connection_manager.establish_connection()?;

            JsonAccessWebTokenBlackListRepository::create(&mut redis_connection_manager, &JsonAccessWebTokenBlackList::new(json_access_web_token.get_id()))?;

            redis_connection_manager.close_connection();

            postgresql_connection_manager.close_connection();

            return Ok(());
        }

        return Err(MainErrorKind::EntityErrorKind(EntityErrorKind::JsonRefreshWebTokenErrorKind(JsonRefreshWebTokenErrorKind::NotFound)));
    }
}