use crate::domain_layer::entity::json_access_web_token::json_access_web_token::JsonAccessWebToken;
use crate::domain_layer::error::entity_error::_component::_in_context_for::domain_layer::entity::json_refresh_web_token::_new_for_context::json_refresh_web_token_error::JsonRefreshWebTokenError;
use crate::domain_layer::error::entity_error::entity_error::EntityError;
use crate::domain_layer::repository::data_provider::_in_context_for::domain_layer::entity::json_refresh_web_token::_new_for_context::_in_context_for::_resource::redis::_new_for_context::base_trait::BaseTrait as DataProviderJsonRefreshWebTokenRedisTrait;
use crate::domain_layer::repository::state_manager::_in_context_for::domain_layer::entity::json_access_web_token_black_list::_new_for_context::_in_context_for::_resource::redis::_new_for_context::base_trait::BaseTrait as StateManagerJsonAccessWebTokenBlackListRedisTrait;
use crate::domain_layer::service::_in_context_for::domain_layer::entity::json_refresh_web_token::_new_for_context::repository_proxy_trait::RepositoryProxyTrait;
use crate::domain_layer::service::factory::_in_context_for::domain_layer::entity::json_access_web_token_black_list::_new_for_context::base_trait::BaseTrait as JsonAccessWebTokenBlackListFactoryTrait;
use crate::infrastructure_layer::error::base_error::base_error::BaseError;
use crate::infrastructure_layer::repository::data_provider::_in_context_for::domain_layer::entity::json_refresh_web_token::_new_for_context::_in_context_for::_resource::redis::_new_for_context::base::Base as DataProviderJsonRefreshWebTokenRedis;
use crate::infrastructure_layer::repository::state_manager::_in_context_for::domain_layer::entity::json_access_web_token_black_list::_new_for_context::_in_context_for::_resource::redis::_new_for_context::base::Base as StateManagerJsonAccessWebTokenBlackListRedis;
use crate::infrastructure_layer::service::_in_context_for::domain_layer::entity::json_refresh_web_token::_new_for_context::repository_proxy::RepositoryProxy;
use crate::infrastructure_layer::service::_in_context_for::infrastructure_layer::repository::_new_for_context::aggregate_connection_pool::AggregateConnectionPool;
use crate::infrastructure_layer::service::_in_context_for::infrastructure_layer::repository::_new_for_context::connection_extractor::ConnectionExtractor;
use crate::infrastructure_layer::service::factory::_in_context_for::domain_layer::entity::json_access_web_token_black_list::_new_for_context::base::Base as JsonAccessWebTokenBlackListFactory;
use redis::Connection;
use std::sync::Arc;

pub struct Base;

impl Base {
    pub fn handle<'outer_a>(
        aggregate_connection_pool: Arc<AggregateConnectionPool>,
        json_access_web_token: &'outer_a JsonAccessWebToken<'_>
    ) -> Result<(), BaseError> {
        let connection: &'_ mut Connection = &mut *ConnectionExtractor::get_redis_connection(&aggregate_connection_pool)?;

        if let Some(json_refresh_web_token) = DataProviderJsonRefreshWebTokenRedis::find_by_application_user_id_and_application_user_log_in_token_device_id(
            connection, json_access_web_token.get_application_user_id(), json_access_web_token.get_application_user_log_in_token_device_id()
        )?
        {
            RepositoryProxy::delete(connection, &json_refresh_web_token)?;

            StateManagerJsonAccessWebTokenBlackListRedis::create(connection, &JsonAccessWebTokenBlackListFactory::create(json_access_web_token.get_id()))?;

            return Ok(());
        }

        return Err(BaseError::EntityError {entity_error: EntityError::JsonRefreshWebTokenError {json_refresh_web_token_error: JsonRefreshWebTokenError::NotFound}});
    }
}