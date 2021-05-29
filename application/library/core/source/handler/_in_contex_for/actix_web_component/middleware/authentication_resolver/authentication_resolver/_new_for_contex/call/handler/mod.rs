use actix_web::dev::ServiceRequest;
use actix_web::HttpMessage;
use actix_web::web::Data;
use crate::error::main_error_kind::core::entity_error_kind::core::_in_context_for::entity::entity::json_access_web_token::_new_for_context::json_access_web_token_error_kind::JsonAccessWebTokenErrorKind;
use crate::error::main_error_kind::core::entity_error_kind::entity_error_kind::EntityErrorKind;
use crate::error::main_error_kind::core::logic_error::LogicError;
use crate::error::main_error_kind::main_error_kind::MainErrorKind;
use crate::repository::_in_context_for::entity::entity::json_access_web_token_black_list::_new_for_context::_in_context_for::_resource::redis::_new_for_context::base_repository::BaseRepository as JsonAccessWebTokenBlackListBaseRepository;
use crate::service::_in_context_for::entity::entity::json_access_web_token::_new_for_context::serialization_form_resolver::SerializationFormResolver;
use crate::utility::_in_context_for::_resource::_new_for_context::aggregate_connection_pool::AggregateConnectionPool;
use crate::utility::_in_context_for::_resource::_new_for_context::connection_extractor::ConnectionExtractor;

pub struct Handler;

impl Handler {
    pub fn handle<'outer_a>(service_request: &'outer_a ServiceRequest) -> Result<(), MainErrorKind> {
        if let Some(data) = service_request.app_data::<Data<AggregateConnectionPool>>() {
            if let Some(header_value) = service_request.headers().get("X-Auth-Token") {
                if let Ok(header_value) = header_value.to_str() {
                    if let Ok(json_access_web_token) = SerializationFormResolver::deserialize(header_value) {
                        if !json_access_web_token.is_expired() {
                            if !JsonAccessWebTokenBlackListBaseRepository::is_exist_by_json_access_token_id(
                                &mut *ConnectionExtractor::get_redis_connection(&data.clone().into_inner())?, json_access_web_token.get_id()
                            )? 
                            {
                                service_request.extensions_mut().insert(json_access_web_token);
    
                                return Ok(());
                            }
    
                            return Err(MainErrorKind::EntityErrorKind(EntityErrorKind::JsonAccessWebTokenErrorKind(JsonAccessWebTokenErrorKind::InJsonAccessWebTokenBlackList)));
                        }
    
                        return Err(MainErrorKind::EntityErrorKind(EntityErrorKind::JsonAccessWebTokenErrorKind(JsonAccessWebTokenErrorKind::AlreadyExpired)));
                    }
                }
            }
    
            return Err(MainErrorKind::InvalidArgumentError);
        }

        return Err(MainErrorKind::LogicError(LogicError::new("'AggregateConnectionPool' must exist in application state")));
    }
}