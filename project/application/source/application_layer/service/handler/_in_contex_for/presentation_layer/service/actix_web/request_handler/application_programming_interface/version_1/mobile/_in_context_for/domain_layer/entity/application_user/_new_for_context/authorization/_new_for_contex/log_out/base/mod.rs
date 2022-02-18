use crate::domain_layer::entity::json_access_web_token_black_list::JsonAccessWebTokenBlackList;
use crate::domain_layer::entity::json_access_web_token::json_access_web_token::JsonAccessWebToken;
use crate::domain_layer::error::entity_error::_component::_in_context_for::domain_layer::entity::json_refresh_web_token::_new_for_context::json_refresh_web_token_error::JsonRefreshWebTokenError;
use crate::domain_layer::error::entity_error::entity_error::EntityError;
use crate::domain_layer::repository::data_provider::_in_context_for::domain_layer::entity::json_refresh_web_token::_new_for_context::_in_context_for::_resource::redis::_new_for_context::base_trait::BaseTrait as JsonRefreshWebTokenDataProviderRedisTrait;
use crate::domain_layer::repository::state_manager::_in_context_for::domain_layer::entity::json_access_web_token_black_list::_new_for_context::_in_context_for::_resource::redis::_new_for_context::base_trait::BaseTrait as JsonAccessWebTokenBlackListStateManagerRedisTrait;
use crate::domain_layer::service::_in_context_for::domain_layer::entity::json_refresh_web_token::_new_for_context::repository_proxy_trait::RepositoryProxyTrait;
use crate::infrastructure_layer::error::base_error::base_error::BaseError;
use crate::infrastructure_layer::repository::data_provider::_in_context_for::domain_layer::entity::json_refresh_web_token::_new_for_context::_in_context_for::_resource::redis::_new_for_context::base::Base as JsonRefreshWebTokenDataProviderRedis;
use crate::infrastructure_layer::repository::state_manager::_in_context_for::domain_layer::entity::json_access_web_token_black_list::_new_for_context::_in_context_for::_resource::redis::_new_for_context::base::Base as JsonAccessWebTokenBlackListStateManagerRedis;
use crate::infrastructure_layer::service::_in_context_for::domain_layer::entity::json_refresh_web_token::_new_for_context::repository_proxy::RepositoryProxy;
use crate::infrastructure_layer::service::_in_context_for::infrastructure_layer::repository::_new_for_context::aggregate_connection_pool::AggregateConnectionPoolXXXxDELETE;
use crate::infrastructure_layer::service::_in_context_for::infrastructure_layer::repository::_new_for_context::connection_extractor::ConnectionExtractorXXXxDelete;
use std::sync::Arc;

pub struct Base;

impl Base {
    pub fn handle<'a>(
        aggregate_connection_pool: Arc<AggregateConnectionPoolXXXxDELETE>,
        json_access_web_token: &'a JsonAccessWebToken<'_>
    ) -> Result<(), BaseError> {
        let redis_connection = &mut *ConnectionExtractorXXXxDelete::get_redis_connection(&aggregate_connection_pool)?;

        if let Some(json_refresh_web_token) = JsonRefreshWebTokenDataProviderRedis::find_by_application_user_id_and_application_user_log_in_token_device_id(
            redis_connection, json_access_web_token.get_application_user_id(), json_access_web_token.get_application_user_log_in_token_device_id()
        )? {
            RepositoryProxy::delete(redis_connection, &json_refresh_web_token)?;

            JsonAccessWebTokenBlackListStateManagerRedis::create(redis_connection, &JsonAccessWebTokenBlackList::new(json_access_web_token.get_id()))?;

            return Ok(());
        }

        return Err(BaseError::EntityError {entity_error: EntityError::JsonRefreshWebTokenError {json_refresh_web_token_error: JsonRefreshWebTokenError::NotFound}});
    }
}