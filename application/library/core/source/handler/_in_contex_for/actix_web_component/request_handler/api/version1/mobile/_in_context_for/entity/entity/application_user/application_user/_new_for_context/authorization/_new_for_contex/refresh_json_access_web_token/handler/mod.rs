// use crate::dto::_in_context_for::actix_web_component::request_handler::api::version1::mobile::_in_context_for::entity::entity::application_user::application_user::_new_for_context::authorization::_new_for_context::refresh_json_access_web_token::request::Request;
// use crate::dto::_in_context_for::handler::_in_context_for::actix_web_component::request_handler::api::version1::mobile::_in_context_for::entity::entity::application_user::application_user::_new_for_context::authorization::_new_for_context::refresh_json_access_web_token::handler::_new_for_context::result::Result as HandlerResult;
// use crate::entity::entity::json_web_token::json_access_web_token::json_access_web_token::JsonAccessWebToken;
// use crate::error::main_error_kind::core::_in_context_for::entity::_new_for_context::entity_error_kind::core::_in_context_for::entity::json_web_token::json_access_web_token::_new_for_context::json_access_web_token_error_kind::JsonAccessWebTokenErrorKind;
// use crate::error::main_error_kind::core::_in_context_for::entity::_new_for_context::entity_error_kind::core::_in_context_for::entity::json_web_token::json_refresh_web_token::_new_for_context::json_refresh_web_token_error_kind::JsonRefreshWebTokenErrorKind;
// use crate::error::main_error_kind::core::_in_context_for::entity::_new_for_context::entity_error_kind::entity_error_kind::EntityErrorKind;
// use crate::error::main_error_kind::main_error_kind::MainErrorKind;
// use crate::repository::_in_context_for::entity::entity::json_web_token::json_refresh_web_token::_new_for_context::postgresql::base_repository::BaseRepository;
// use crate::service::_in_context_for::entity::entity::json_web_token::json_access_web_token::_new_for_context::serialization_form_resolver::SerializationFormResolver;
// use crate::utility::_in_context_for::diesel_component::_new_for_context::postgresql::connection_manager::ConnectionManager;

// pub struct Handler;

// impl Handler {
//     pub fn handle(request: Request) -> Result<HandlerResult, MainErrorKind> {
//         let json_access_web_token: JsonAccessWebToken<'_> = SerializationFormResolver::deserialize(request.json_access_web_token.as_str())?;

//         if json_access_web_token.is_expired() {
//             let mut connection_manager: ConnectionManager = ConnectionManager::new();
//             connection_manager.establish_connection()?;

//             match BaseRepository::get_by_application_user_id_and_application_user_log_in_token_device_id(
//                 &connection_manager, json_access_web_token.get_application_user_id(), json_access_web_token.get_application_user_log_in_token_device_id()
//             )? 
//             {
//                 Some (json_refresh_web_token) => {
//                     if json_refresh_web_token.get_value().get_value() == json_access_web_token.get_json_refresh_web_token_value().get_value() {
//                         json_refresh_web_token.refresh_expired_at()
//                     } else {
//                         return Err(EntityErrorKind::JsonRefreshWebTokenErrorKind(JsonRefreshWebTokenErrorKind::InvalidValue))?
//                     }
//                 },
//                 None => {
//                     return Err(EntityErrorKind::JsonRefreshWebTokenErrorKind(JsonRefreshWebTokenErrorKind::NotExist))?
//                 }
//             };
//         } else {
//             return Err(EntityErrorKind::JsonAccessWebTokenErrorKind(JsonAccessWebTokenErrorKind::NotExpired))?;
//         }
//     }
// }