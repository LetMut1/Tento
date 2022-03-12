use bb8_redis::RedisConnectionManager;
use bb8::Pool;
use crate::domain_layer::entity::json_access_web_token_black_list::JsonAccessWebTokenBlackList;
use crate::domain_layer::error::entity_error::_component::_in_context_for::domain_layer::entity::json_refresh_web_token::_new_for_context::json_refresh_web_token_error::JsonRefreshWebTokenError;
use crate::domain_layer::error::entity_error::entity_error::EntityError;
use crate::infrastructure_layer::error::base_error::base_error::BaseError;
use crate::infrastructure_layer::repository::state_manager::_in_context_for::domain_layer::entity::json_access_web_token_black_list::_new_for_context::_in_context_for::_resource::redis::_new_for_context::base::Base as JsonAccessWebTokenBlackListStateManagerRedis;
use crate::infrastructure_layer::service::_in_context_for::domain_layer::entity::json_access_web_token::_new_for_context::extractor::Extractor;
use crate::infrastructure_layer::service::_in_context_for::domain_layer::entity::json_refresh_web_token::_new_for_context::repository_proxy::RepositoryProxy;
use crate::presentation_layer::data_transfer_object::request::_in_context_for::presentation_layer::service::request_handler::application_programming_interface::version_1::mobile::_in_context_for::domain_layer::entity::application_user::_new_for_context::authorization::_new_for_context::log_out_from_all_devices::base::Base as Request;

pub struct Base;

impl Base {
    pub async fn handle<'a>(
        redis_connection_pool: Pool<RedisConnectionManager>,
        request: Request
    ) -> Result<(), BaseError> {
        let redis_connection  = &mut *redis_connection_pool.get().await?;

        let json_access_web_token = request.into_inner();
        let json_access_web_token_ = Extractor::extract(json_access_web_token.as_str(), redis_connection).await?;
        if let Some(json_refresh_web_token_registry) = RepositoryProxy::get_by_application_user_id(
            redis_connection, json_access_web_token_.get_application_user_id()
        ).await? {
            for json_refresh_web_token in json_refresh_web_token_registry.iter() {
                RepositoryProxy::delete(redis_connection, json_refresh_web_token).await?;

                JsonAccessWebTokenBlackListStateManagerRedis::create(redis_connection, &JsonAccessWebTokenBlackList::new(json_access_web_token_.get_id())).await?;
            }
            
            return Ok(());
        }

        return Err(BaseError::EntityError {entity_error: EntityError::JsonRefreshWebTokenError {json_refresh_web_token_error: JsonRefreshWebTokenError::NotFound}});
    }
}