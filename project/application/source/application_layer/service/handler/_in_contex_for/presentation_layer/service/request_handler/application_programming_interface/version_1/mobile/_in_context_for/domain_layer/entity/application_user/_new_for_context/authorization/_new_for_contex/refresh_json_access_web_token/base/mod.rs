use bb8_redis::RedisConnectionManager;
use bb8::Pool;
use crate::domain_layer::error::entity_error::_component::_in_context_for::domain_layer::entity::json_access_web_token::_new_for_context::json_access_web_token_error::JsonAccessWebTokenError;
use crate::domain_layer::error::entity_error::_component::_in_context_for::domain_layer::entity::json_refresh_web_token::_new_for_context::json_refresh_web_token_error::JsonRefreshWebTokenError;
use crate::domain_layer::error::entity_error::entity_error::EntityError;
use crate::domain_layer::service::_in_context_for::domain_layer::entity::json_access_web_token::_new_for_context::expiration_time_resolver_trait::ExpirationTimeResolverTrait;
use crate::domain_layer::service::_in_context_for::domain_layer::entity::json_access_web_token::_new_for_context::serialization_form_resolver_trait::SerializationFormResolverTrait;
use crate::domain_layer::service::_in_context_for::domain_layer::entity::json_refresh_web_token::_new_for_context::encoder_trait::EncoderTrait;
use crate::domain_layer::service::_in_context_for::domain_layer::entity::json_refresh_web_token::_new_for_context::refresher_trait::RefresherTrait;
use crate::domain_layer::service::factory::_in_context_for::domain_layer::entity::json_access_web_token::_new_for_context::base_trait::BaseTrait as JsonAccessWebTokenFactoryTrait;
use crate::infrastructure_layer::error::base_error::base_error::BaseError;
use crate::infrastructure_layer::repository::data_provider::_in_context_for::domain_layer::entity::json_refresh_web_token::_new_for_context::_in_context_for::_resource::redis::_new_for_context::base::Base as JsonRefreshWebTokenDataProviderRedis;
use crate::infrastructure_layer::service::_in_context_for::domain_layer::entity::json_access_web_token::_new_for_context::expiration_time_resolver::ExpirationTimeResolver;
use crate::infrastructure_layer::service::_in_context_for::domain_layer::entity::json_access_web_token::_new_for_context::serialization_form_resolver::SerializationFormResolver;
use crate::infrastructure_layer::service::_in_context_for::domain_layer::entity::json_refresh_web_token::_new_for_context::encoder::Encoder;
use crate::infrastructure_layer::service::_in_context_for::domain_layer::entity::json_refresh_web_token::_new_for_context::refresher::Refresher;
use crate::infrastructure_layer::service::_in_context_for::domain_layer::entity::json_refresh_web_token::_new_for_context::repository_proxy::RepositoryProxy;
use crate::infrastructure_layer::service::factory::_in_context_for::domain_layer::entity::json_access_web_token::_new_for_context::base::Base as JsonAccessWebTokenFactory;
use crate::presentation_layer::data_transfer_object::request::_in_context_for::presentation_layer::service::request_handler::application_programming_interface::version_1::mobile::_in_context_for::domain_layer::entity::application_user::_new_for_context::authorization::_new_for_context::refresh_json_access_web_token::base::Base as Request;
use crate::presentation_layer::data_transfer_object::response::_in_context_for::presentation_layer::service::request_handler::application_programming_interface::version_1::mobile::_in_context_for::domain_layer::entity::application_user::_new_for_context::authorization::_new_for_context::refresh_json_access_web_token::base::Base as Response;

pub struct Base;

impl Base {
    pub async fn handle(
        redis_connection_pool: Pool<RedisConnectionManager>,
        request: Request
    ) -> Result<Response, BaseError> {
        let (
            json_access_web_token, 
            json_refresh_web_token
        ) = request.into_inner();

        let json_access_web_token = SerializationFormResolver::deserialize(json_access_web_token.as_str())?;

        if ExpirationTimeResolver::is_expired(&json_access_web_token)? {        // TODO TODO TODO TODO СДелать интервал, когда можео менять. На 3 часа раньше, чем срок экспирации, например
            let redis_connection = &mut *redis_connection_pool.get().await?;

            if let Some(mut json_refresh_web_token_) = JsonRefreshWebTokenDataProviderRedis::find_by_application_user_id_and_application_user_log_in_token_device_id(
                redis_connection, json_access_web_token.get_application_user_id(), json_access_web_token.get_application_user_log_in_token_device_id()
            ).await? {
                if json_access_web_token.get_id().as_bytes()[..] == json_refresh_web_token_.get_json_access_web_token_id().as_bytes()[..] 
                    && Encoder::is_valid(&json_refresh_web_token_, json_refresh_web_token.as_str())? {
                    Refresher::refresh(&mut json_refresh_web_token_);

                    RepositoryProxy::update(redis_connection, &json_refresh_web_token_).await?;

                    let json_access_web_token = SerializationFormResolver::serialize(
                        &JsonAccessWebTokenFactory::create_from_json_refresh_web_token(&json_refresh_web_token_)?
                    )?;

                    let json_refresh_web_token = Encoder::encode(&json_refresh_web_token_)?;

                    return Ok(Response::new(json_access_web_token, json_refresh_web_token));
                }

                return Err(BaseError::InvalidArgumentError);
            }

            return Err(BaseError::EntityError {entity_error: EntityError::JsonRefreshWebTokenError {json_refresh_web_token_error: JsonRefreshWebTokenError::NotFound}});
        }
        
        return Err(BaseError::EntityError {entity_error: EntityError::JsonAccessWebTokenError {json_access_web_token_error: JsonAccessWebTokenError::NotExpired}});
    }
}