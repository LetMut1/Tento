use actix_web::dev::ServiceRequest;
use actix_web::HttpMessage;
use actix_web::web::Data;
use crate::domain_layer::entity::json_access_web_token::json_access_web_token::JsonAccessWebToken;
use crate::domain_layer::error::entity_error::_component::_in_context_for::domain_layer::entity::json_access_web_token::_new_for_context::json_access_web_token_error::JsonAccessWebTokenError;
use crate::domain_layer::error::entity_error::entity_error::EntityError;
use crate::domain_layer::repository::data_provider::_in_context_for::domain_layer::entity::json_access_web_token_black_list::_new_for_context::_in_context_for::_resource::redis::_new_for_context::base_trait::BaseTrait as JsonAccessWebTokenBlackListDataProviderRedisTrait;
use crate::domain_layer::service::_in_context_for::domain_layer::entity::json_access_web_token::_new_for_context::expiration_time_resolver_trait::ExpirationTimeResolverTrait;
use crate::domain_layer::service::_in_context_for::domain_layer::entity::json_access_web_token::_new_for_context::serialization_form_resolver_trait::SerializationFormResolverTrait;
use crate::infrastructure_layer::error::base_error::_component::logic_error::LogicError;
use crate::infrastructure_layer::error::base_error::base_error::BaseError;
use crate::infrastructure_layer::repository::data_provider::_in_context_for::domain_layer::entity::json_access_web_token_black_list::_new_for_context::_in_context_for::_resource::redis::_new_for_context::base::Base as JsonAccessWebTokenBlackListDataProviderRedis;
use crate::infrastructure_layer::service::_in_context_for::domain_layer::entity::json_access_web_token::_new_for_context::expiration_time_resolver::ExpirationTimeResolver;
use crate::infrastructure_layer::service::_in_context_for::domain_layer::entity::json_access_web_token::_new_for_context::serialization_form_resolver::SerializationFormResolver;
use crate::infrastructure_layer::service::_in_context_for::infrastructure_layer::repository::_new_for_context::aggregate_connection_pool::AggregateConnectionPool;
use crate::infrastructure_layer::service::_in_context_for::infrastructure_layer::repository::_new_for_context::connection_extractor::ConnectionExtractor;

pub struct Base;

impl Base {
    pub fn handle<'a>(
        service_request: &'a ServiceRequest
    ) -> Result<(), BaseError> {
        if let Some(aggregate_connection_pool) = service_request.app_data::<Data<AggregateConnectionPool>>() {
            if let Some(x_auth_token_header_value) = service_request.headers().get("X-Jawt") {
                if let Ok(header_value) = x_auth_token_header_value.to_str() {
                    let json_access_web_token: JsonAccessWebToken<'_> = SerializationFormResolver::deserialize(header_value)?;
                    if !ExpirationTimeResolver::is_expired(&json_access_web_token)? {
                        if !JsonAccessWebTokenBlackListDataProviderRedis::is_exist_by_json_access_token_id(
                            &mut *ConnectionExtractor::get_redis_connection(&aggregate_connection_pool.clone().into_inner())?, json_access_web_token.get_id()
                        )? {
                            service_request.extensions_mut().insert(json_access_web_token);     // TODO В такой реализации При инъекции в РекуестХэндлер этот объкт будет склонирован. А нужно передать его самого. Актикс сейчас не позволякет это сделать. Проверить позже

                            return Ok(());
                        }

                        return Err(BaseError::EntityError {entity_error: EntityError::JsonAccessWebTokenError {json_access_web_token_error: JsonAccessWebTokenError::InJsonAccessWebTokenBlackList}});
                    }

                    return Err(BaseError::EntityError {entity_error: EntityError::JsonAccessWebTokenError {json_access_web_token_error: JsonAccessWebTokenError::AlreadyExpired}});
                }
            }
    
            return Err(BaseError::EntityError {entity_error: EntityError::JsonAccessWebTokenError {json_access_web_token_error: JsonAccessWebTokenError::NotFound}});
        }

        return Err(BaseError::LogicError {logic_error: LogicError::new(true, "Aggregate_connection_pool must exist in application state.")});
    }
}