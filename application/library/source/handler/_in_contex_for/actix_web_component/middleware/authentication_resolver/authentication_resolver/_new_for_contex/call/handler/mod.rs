use actix_web::dev::ServiceRequest;
use actix_web::HttpMessage;
use actix_web::web::Data;
use crate::error::main_error::core::entity_error::core::_in_context_for::entity::entity::json_access_web_token::_new_for_context::json_access_web_token_error::JsonAccessWebTokenError;
use crate::error::main_error::core::entity_error::entity_error::EntityError;
use crate::error::main_error::core::logic_error::LogicError;
use crate::error::main_error::main_error::MainError;
use crate::repository::_in_context_for::entity::entity::json_access_web_token_black_list::_new_for_context::_in_context_for::_resource::redis::_new_for_context::base_repository::BaseRepository as JsonAccessWebTokenBlackListBaseRepository;
use crate::service::_in_context_for::entity::entity::json_access_web_token::_new_for_context::serialization_form_resolver::SerializationFormResolver;
use crate::utility::_in_context_for::_resource::_new_for_context::aggregate_connection_pool::AggregateConnectionPool;
use crate::utility::_in_context_for::_resource::_new_for_context::connection_extractor::ConnectionExtractor;

pub struct Handler;

impl Handler {
    pub fn handle<'outer_a>(service_request: &'outer_a ServiceRequest) -> Result<(), MainError> {
        if let Some(data) = service_request.app_data::<Data<AggregateConnectionPool>>() {
            if let Some(header_value) = service_request.headers().get("X-Auth-Token") {
                if let Ok(header_value) = header_value.to_str() {
                    if let Ok(json_access_web_token) = SerializationFormResolver::deserialize(header_value) {
                        if !json_access_web_token.is_expired() {
                            if !JsonAccessWebTokenBlackListBaseRepository::is_exist_by_json_access_token_id(
                                &mut *ConnectionExtractor::get_redis_connection(&data.clone().into_inner())?, json_access_web_token.get_id()
                            )? 
                            {
                                service_request.extensions_mut().insert(json_access_web_token);     // TODO В такой реализации При инъекции в РекуестХэндлер этот объкт будет склонирован. А нужно передать его самого. Актикс сейчас не позволякет это сделать. Проверить позже
    
                                return Ok(());
                            }
    
                            return Err(MainError::EntityError(EntityError::JsonAccessWebTokenError(JsonAccessWebTokenError::InJsonAccessWebTokenBlackList)));
                        }
    
                        return Err(MainError::EntityError(EntityError::JsonAccessWebTokenError(JsonAccessWebTokenError::AlreadyExpired)));
                    }
                }
            }
    
            return Err(MainError::InvalidArgumentError);
        }

        return Err(MainError::LogicError(LogicError::new("'AggregateConnectionPool' must exist in application state")));
    }
}