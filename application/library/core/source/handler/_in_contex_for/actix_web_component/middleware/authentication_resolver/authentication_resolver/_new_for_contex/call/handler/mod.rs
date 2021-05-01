use actix_web::dev::ServiceRequest;
use actix_web::HttpMessage;
use crate::error::main_error_kind::core::entity_error_kind::core::_in_context_for::entity::entity::json_access_web_token::_new_for_context::json_access_web_token_error_kind::JsonAccessWebTokenErrorKind;
use crate::error::main_error_kind::core::entity_error_kind::entity_error_kind::EntityErrorKind;
use crate::error::main_error_kind::main_error_kind::MainErrorKind;
use crate::repository::_in_context_for::entity::entity::json_access_web_token_black_list::_new_for_context::redis::base_repository::BaseRepository as JsonAccessWebTokenBlackListBaseRepository;
use crate::service::_in_context_for::entity::entity::json_access_web_token::_new_for_context::serialization_form_resolver::SerializationFormResolver;
use crate::utility::resource_connection::redis::connection_manager::ConnectionManager;

pub struct Handler;

impl<'outer> Handler {
    pub fn handle(service_request: &'outer ServiceRequest) -> Result<(), MainErrorKind> {
        if let Some(header_value) = service_request.headers().get("X-Auth-Token") {
            if let Ok(header_value) = header_value.to_str() {
                if let Ok(json_access_web_token) = SerializationFormResolver::deserialize(header_value) {
                    if !json_access_web_token.is_expired() {
                        let mut connection_manager: ConnectionManager = ConnectionManager::new();
                        connection_manager.establish_connection()?;

                        if !JsonAccessWebTokenBlackListBaseRepository::is_exist_by_json_access_token_id(
                            &mut connection_manager, json_access_web_token.get_id()
                        )? 
                        {
                            connection_manager.close_connection();

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
}