use crate::entity::entity::json_access_web_token_black_list::json_access_web_token_black_list::JsonAccessWebTokenBlackList;
use crate::entity::entity::json_access_web_token::json_access_web_token::JsonAccessWebToken;
use crate::error::main_error::_core::entity_error::_core::_in_context_for::entity::entity::json_refresh_web_token::_new_for_context::json_refresh_web_token_error::JsonRefreshWebTokenError;
use crate::error::main_error::_core::entity_error::entity_error::EntityError;
use crate::error::main_error::main_error::MainError;
use crate::repository::_in_context_for::entity::entity::json_access_web_token_black_list::_new_for_context::_in_context_for::_resource::redis::_new_for_context::base_repository::BaseRepository as JsonAccessWebTokenBlackListRepository;
use crate::service::_in_context_for::entity::entity::json_refresh_web_token::_new_for_context::base_repository_proxy::BaseRepositoryProxy;
use crate::utility::_in_context_for::_resource::_new_for_context::aggregate_connection_pool::AggregateConnectionPool;
use crate::utility::_in_context_for::_resource::_new_for_context::connection_extractor::ConnectionExtractor;
use redis::Connection;
use std::sync::Arc;

pub struct Handler;

impl Handler {
    pub fn handle<'outer_a>(
        aggregate_connection_pool: Arc<AggregateConnectionPool>, json_access_web_token: &'outer_a JsonAccessWebToken<'_>
    ) -> Result<(), MainError> {
        let connection: &'_ mut Connection = &mut *ConnectionExtractor::get_redis_connection(&aggregate_connection_pool)?;

        if let Some(json_refresh_web_token_registry) = BaseRepositoryProxy::get_by_application_user_id(
            connection, json_access_web_token.get_application_user_id()
        )?
        {
            for json_refresh_web_token in json_refresh_web_token_registry.iter() {
                BaseRepositoryProxy::delete(connection, json_refresh_web_token)?;

                JsonAccessWebTokenBlackListRepository::create(connection, &JsonAccessWebTokenBlackList::new(json_access_web_token.get_id()))?;
            }
            
            return Ok(());
        }

        return Err(MainError::EntityError(EntityError::JsonRefreshWebTokenError(JsonRefreshWebTokenError::NotFound)));
    }
}