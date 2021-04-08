use crate::entity::entity::json_access_web_token_black_list::json_access_web_token_black_list::JsonAccessWebTokenBlackList;
use crate::entity::entity::json_web_token::json_access_web_token::json_access_web_token::JsonAccessWebToken;
use crate::error::main_error_kind::core::_in_context_for::entity::_new_for_context::entity_error_kind::entity_error_kind::EntityErrorKind;
use crate::error::main_error_kind::main_error_kind::MainErrorKind;
use crate::repository::_in_context_for::entity::entity::json_access_web_token_black_list::_new_for_context::postgresql::base_repository::BaseRepository as JsonAccessWebTokenBlackListRepository;
use crate::repository::_in_context_for::entity::entity::json_web_token::json_refresh_web_token::_new_for_context::postgresql::base_repository::BaseRepository as JsonRefreshWebTokenBaseRepository;
use crate::utility::_in_context_for::diesel_component::_new_for_context::postgresql::connection_manager::ConnectionManager;
use crate::error::main_error_kind::core::_in_context_for::entity::_new_for_context::entity_error_kind::core::_in_context_for::entity::json_web_token::json_refresh_web_token::_new_for_context::json_refresh_web_token_error_kind::JsonRefreshWebTokenErrorKind;

pub struct Handler;

// impl<'outer> Handler {
//     pub fn handle(json_access_web_token: &'outer JsonAccessWebToken<'outer>) -> Result<(), MainErrorKind> {
//         let mut connection_manager: ConnectionManager = ConnectionManager::new();
//         connection_manager.establish_connection()?;

//         // if let Some(json_refresh_web_token) = JsonRefreshWebTokenBaseRepository::get_by_application_user_id_and_application_user_log_in_token_device_id(
//         //     &connection_manager, json_access_web_token.get_application_user_id(), json_access_web_token.get_application_user_log_in_token_device_id()
//         // )?
//         // {
//         //     // TODO без транзакции, так как все будет на кеше (Удалить это сообщение, как только перепишу на Кеш)
//         //     JsonRefreshWebTokenBaseRepository::delete(&connection_manager, &json_refresh_web_token)?;

//         //     JsonAccessWebTokenBlackListRepository::create(&connection_manager, &JsonAccessWebTokenBlackList::new(&json_access_web_token.get_id()))?;

//         //     return Ok(());
//         // }

//         // return Err(EntityErrorKind::JsonRefreshWebTokenErrorKind(JsonRefreshWebTokenErrorKind::NotFound))?;
//     }
// }