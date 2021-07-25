use crate::domain_layer::entity::entity::json_access_web_token_black_list::json_access_web_token_black_list::JsonAccessWebTokenBlackList;
use crate::domain_layer::entity::entity::json_access_web_token::json_access_web_token::JsonAccessWebToken;
use crate::domain_layer::error::entity_error::_core::_in_context_for::domain_layer::entity::entity::json_refresh_web_token::_new_for_context::json_refresh_web_token_error::JsonRefreshWebTokenError;
use crate::domain_layer::error::entity_error::entity_error::EntityError;
use crate::domain_layer::service::_in_context_for::domain_layer::entity::entity::json_refresh_web_token::_new_for_context::base_repository_proxy_trait::BaseRepositoryProxyTrait;
use crate::infrastructure_layer::error::base_error::base_error::BaseError;
use crate::infrastructure_layer::repository::_in_context_for::entity::entity::json_access_web_token_black_list::_new_for_context::_in_context_for::_resource::redis::_new_for_context::base_repository::BaseRepository as JsonAccessWebTokenBlackListRepository;
use crate::infrastructure_layer::service::_in_context_for::_resource::_new_for_context::aggregate_connection_pool::AggregateConnectionPool;
use crate::infrastructure_layer::service::_in_context_for::_resource::_new_for_context::connection_extractor::ConnectionExtractor;
use crate::infrastructure_layer::service::_in_context_for::domain_layer::entity::entity::json_refresh_web_token::_new_for_context::base_repository_proxy::BaseRepositoryProxy;
use redis::Connection;
use std::sync::Arc;

pub struct Handler;

impl Handler {
    pub fn handle<'outer_a>(
        aggregate_connection_pool: Arc<AggregateConnectionPool>, json_access_web_token: &'outer_a JsonAccessWebToken<'_>
    ) -> Result<(), BaseError> {
        let connection: &'_ mut Connection = &mut *ConnectionExtractor::get_redis_connection(&aggregate_connection_pool)?;

        if let Some(json_refresh_web_token) = <BaseRepositoryProxy as BaseRepositoryProxyTrait>::get_by_application_user_id_and_application_user_log_in_token_device_id(
            connection, json_access_web_token.get_application_user_id(), json_access_web_token.get_application_user_log_in_token_device_id()
        )?
        {
            <BaseRepositoryProxy as BaseRepositoryProxyTrait>::delete(connection, &json_refresh_web_token)?;

            JsonAccessWebTokenBlackListRepository::create(connection, &JsonAccessWebTokenBlackList::new(json_access_web_token.get_id()))?;

            return Ok(());
        }

        return Err(BaseError::EntityError(EntityError::JsonRefreshWebTokenError(JsonRefreshWebTokenError::NotFound)));
    }
}