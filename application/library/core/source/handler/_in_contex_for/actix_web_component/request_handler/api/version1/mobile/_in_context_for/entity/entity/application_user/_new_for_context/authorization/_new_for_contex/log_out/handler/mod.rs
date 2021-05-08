use crate::entity::entity::json_access_web_token_black_list::json_access_web_token_black_list::JsonAccessWebTokenBlackList;
use crate::entity::entity::json_access_web_token::json_access_web_token::JsonAccessWebToken;
use crate::error::main_error_kind::core::entity_error_kind::core::_in_context_for::entity::entity::json_refresh_web_token::_new_for_context::json_refresh_web_token_error_kind::JsonRefreshWebTokenErrorKind;
use crate::error::main_error_kind::core::entity_error_kind::entity_error_kind::EntityErrorKind;
use crate::error::main_error_kind::main_error_kind::MainErrorKind;
use crate::repository::_in_context_for::entity::entity::json_access_web_token_black_list::_new_for_context::_in_context_for::_resource::redis::_new_for_context::base_repository::BaseRepository as JsonAccessWebTokenBlackListRepository;
use crate::service::_in_context_for::entity::entity::json_refresh_web_token::_new_for_context::base_repository_proxy::BaseRepositoryProxy;
use crate::utility::_in_context_for::_resource::redis::_new_for_context::connection_manager::ConnectionManager;

pub struct Handler;

impl<'outer_a> Handler {
    pub fn handle(json_access_web_token: &'outer_a JsonAccessWebToken<'outer_a>) -> Result<(), MainErrorKind> {
        let mut connection_manager: ConnectionManager = ConnectionManager::new();
        connection_manager.establish_connection()?;

        if let Some(json_refresh_web_token) = BaseRepositoryProxy::get_by_application_user_id_and_application_user_log_in_token_device_id(
            &mut connection_manager, json_access_web_token.get_application_user_id(), json_access_web_token.get_application_user_log_in_token_device_id()
        )?
        {
            BaseRepositoryProxy::delete(&mut connection_manager, &json_refresh_web_token)?;

            JsonAccessWebTokenBlackListRepository::create(&mut connection_manager, &JsonAccessWebTokenBlackList::new(json_access_web_token.get_id()))?;

            connection_manager.close_connection();

            return Ok(());
        }

        return Err(MainErrorKind::EntityErrorKind(EntityErrorKind::JsonRefreshWebTokenErrorKind(JsonRefreshWebTokenErrorKind::NotFound)));
    }
}