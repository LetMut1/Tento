use crate::domain_layer::entity::entity::json_access_web_token::json_access_web_token::JsonAccessWebToken;
use crate::domain_layer::error::entity_error::_core::_in_context_for::domain_layer::entity::entity::json_access_web_token::_new_for_context::json_access_web_token_error::JsonAccessWebTokenError;
use crate::domain_layer::error::entity_error::_core::_in_context_for::domain_layer::entity::entity::json_refresh_web_token::_new_for_context::json_refresh_web_token_error::JsonRefreshWebTokenError;
use crate::domain_layer::error::entity_error::entity_error::EntityError;
use crate::domain_layer::service::_in_context_for::domain_layer::entity::entity::json_access_web_token::_new_for_context::serialization_form_resolver_trait::SerializationFormResolverTrait;
use crate::domain_layer::service::_in_context_for::domain_layer::entity::entity::json_refresh_web_token::_new_for_context::base_repository_proxy_trait::BaseRepositoryProxyTrait;
use crate::domain_layer::service::_in_context_for::domain_layer::entity::entity::json_refresh_web_token::_new_for_context::encoder_trait::EncoderTrait;
use crate::infrastructure_layer::error::base_error::base_error::BaseError;
use crate::infrastructure_layer::service::_in_context_for::_resource::_new_for_context::aggregate_connection_pool::AggregateConnectionPool;
use crate::infrastructure_layer::service::_in_context_for::_resource::_new_for_context::connection_extractor::ConnectionExtractor;
use crate::infrastructure_layer::service::_in_context_for::domain_layer::entity::entity::json_access_web_token::_new_for_context::serialization_form_resolver::SerializationFormResolver;
use crate::infrastructure_layer::service::_in_context_for::domain_layer::entity::entity::json_refresh_web_token::_new_for_context::base_repository_proxy::BaseRepositoryProxy;
use crate::infrastructure_layer::service::_in_context_for::domain_layer::entity::entity::json_refresh_web_token::_new_for_context::encoder::Encoder;
use crate::presentation_layer::data_transfer_object::request::_in_context_for::presentation_layer::service::actix_web_component::request_handler::api::version1::mobile::_in_context_for::domain_layer::entity::entity::application_user::_new_for_context::authorization::_new_for_context::refresh_json_access_web_token::request::Request;
use crate::presentation_layer::data_transfer_object::response::_in_context_for::presentation_layer::service::actix_web_component::request_handler::api::version1::mobile::_in_context_for::domain_layer::entity::entity::application_user::_new_for_context::authorization::_new_for_context::refresh_json_access_web_token::response::Response;
use redis::Connection;
use std::sync::Arc;

pub struct Handler;

impl Handler {
    pub fn handle(aggregate_connection_pool: Arc<AggregateConnectionPool>, request: Request) -> Result<Response, BaseError> {
        let (
            json_access_web_token, 
            json_refresh_web_token_serialized
        ) = request.into_inner();

        let json_access_web_token: JsonAccessWebToken<'_> = SerializationFormResolver::deserialize(json_access_web_token.as_str())?;

        if json_access_web_token.is_expired() {
            let connection: &'_ mut Connection = &mut *ConnectionExtractor::get_redis_connection(&aggregate_connection_pool)?;

            if let Some(mut json_refresh_web_token) = BaseRepositoryProxy::get_by_application_user_id_and_application_user_log_in_token_device_id(
                connection, json_access_web_token.get_application_user_id(), json_access_web_token.get_application_user_log_in_token_device_id()
            )?
            {
                if &(json_access_web_token.get_id().get_value().get_value().as_bytes())[..] == &(json_refresh_web_token.get_json_access_web_token_id().get_value().get_value().as_bytes())[..] 
                && Encoder::is_valid(&json_refresh_web_token, json_refresh_web_token_serialized.as_str())? 
                {
                    json_refresh_web_token.refresh();

                    BaseRepositoryProxy::update(connection, &json_refresh_web_token)?;

                    return Ok(
                        Response::new(
                            SerializationFormResolver::serialize(&JsonAccessWebToken::new(&json_refresh_web_token)?)?,
                            Encoder::encode(&json_refresh_web_token)?
                        )
                    );
                }

                return Err(BaseError::InvalidArgumentError);
            }

            return Err(BaseError::EntityError(EntityError::JsonRefreshWebTokenError(JsonRefreshWebTokenError::NotFound)));
        }
        
        return Err(BaseError::EntityError(EntityError::JsonAccessWebTokenError(JsonAccessWebTokenError::NotExpired)));
    }
}