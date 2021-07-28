use actix_web::dev::ServiceRequest;
use actix_web::HttpMessage;
use actix_web::web::Data;
use crate::domain_layer::entity::entity::json_access_web_token::json_access_web_token::JsonAccessWebToken;
use crate::domain_layer::error::entity_error::_component::_in_context_for::domain_layer::entity::entity::json_access_web_token::_new_for_context::json_access_web_token_error::JsonAccessWebTokenError;
use crate::domain_layer::error::entity_error::entity_error::EntityError;
use crate::domain_layer::repository::_in_context_for::domain_layer::entity::entity::json_access_web_token_black_list::_new_for_context::_in_context_for::_resource::redis::_new_for_context::base_repository_trait::BaseRepositoryTrait as JsonAccessWebTokenBlackListBaseRepositoryTrait;
use crate::domain_layer::service::_in_context_for::domain_layer::entity::entity::json_access_web_token::_new_for_context::serialization_form_resolver_trait::SerializationFormResolverTrait;
use crate::infrastructure_layer::error::base_error::base_error::BaseError;
use crate::infrastructure_layer::repository::_in_context_for::domain_layer::entity::entity::json_access_web_token_black_list::_new_for_context::_in_context_for::_resource::redis::_new_for_context::base_repository::BaseRepository as JsonAccessWebTokenBlackListBaseRepository;
use crate::infrastructure_layer::service::_in_context_for::_resource::_new_for_context::aggregate_connection_pool::AggregateConnectionPool;
use crate::infrastructure_layer::service::_in_context_for::_resource::_new_for_context::connection_extractor::ConnectionExtractor;
use crate::infrastructure_layer::service::_in_context_for::domain_layer::entity::entity::json_access_web_token::_new_for_context::serialization_form_resolver::SerializationFormResolver;

pub struct Handler;

impl Handler {
    pub fn handle<'outer_a>(service_request: &'outer_a ServiceRequest) -> Result<(), BaseError> {
        if let Some(data) = service_request.app_data::<Data<AggregateConnectionPool>>() {
            if let Some(header_value) = service_request.headers().get("X-Auth-Token") {
                if let Ok(header_value) = header_value.to_str() {
                    let json_access_web_token: JsonAccessWebToken<'_> = SerializationFormResolver::deserialize(header_value)?;
                    if !json_access_web_token.is_expired() {
                        if !JsonAccessWebTokenBlackListBaseRepository::is_exist_by_json_access_token_id(
                            &mut *ConnectionExtractor::get_redis_connection(&data.clone().into_inner())?, json_access_web_token.get_id()
                        )? 
                        {
                            service_request.extensions_mut().insert(json_access_web_token);     // TODO В такой реализации При инъекции в РекуестХэндлер этот объкт будет склонирован. А нужно передать его самого. Актикс сейчас не позволякет это сделать. Проверить позже

                            return Ok(());
                        }

                        return Err(BaseError::EntityError(EntityError::JsonAccessWebTokenError(JsonAccessWebTokenError::InJsonAccessWebTokenBlackList)));
                    }

                    return Err(BaseError::EntityError(EntityError::JsonAccessWebTokenError(JsonAccessWebTokenError::AlreadyExpired)));
                }
            }
    
            return Err(BaseError::InvalidArgumentError);
        }

        return Err(BaseError::LogicError("'AggregateConnectionPool' must exist in application state."));
    }
}