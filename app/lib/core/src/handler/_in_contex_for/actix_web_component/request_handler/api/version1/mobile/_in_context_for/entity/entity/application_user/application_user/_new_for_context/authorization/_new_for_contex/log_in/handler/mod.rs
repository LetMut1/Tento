use crate::dto::_in_context_for::actix_web_component::request_handler::api::version1::mobile::_in_context_for::entity::entity::application_user::application_user::_new_for_context::authorization::_new_for_context::log_in::request::Request;
use crate::dto::_in_context_for::handler::_in_context_for::actix_web_component::request_handler::api::version1::mobile::_in_context_for::entity::entity::application_user::application_user::_new_for_context::authorization::_new_for_context::log_in::handler::_new_for_context::result::Result as HandlerResult;
use crate::entity::entity::json_web_token::json_access_web_token::json_access_web_token::JsonAccessWebToken;
use crate::entity::entity::json_web_token::json_refresh_web_token::json_refresh_web_token::JsonRefreshWebToken;
use crate::error::main_error_kind::core::_in_context_for::entity::_new_for_context::entity_error_kind::core::_in_context_for::entity::application_user_log_in_token::_new_for_context::application_user_log_in_token::ApplicationUserLogInTokenErrorKind;
use crate::error::main_error_kind::core::_in_context_for::entity::_new_for_context::entity_error_kind::entity_error_kind::EntityErrorKind;
use crate::error::main_error_kind::main_error_kind::MainErrorKind;
use crate::repository::_in_context_for::entity::entity::application_user_log_in_token::_new_for_context::postgresql::base_repository::BaseRepository as ApplicationUserLogInTokenBaseRepository;
use crate::repository::_in_context_for::entity::entity::json_web_token::json_refresh_web_token::_new_for_context::postgresql::base_repository::BaseRepository as JsonRefreshWebTokenBaseRepository;
use crate::service::_in_context_for::entity::entity::json_web_token::json_access_web_token::_new_for_context::serialization_form_resolver::SerializationFormResolver;
use crate::utility::_in_context_for::diesel_component::_new_for_context::postgresql::connection_manager::ConnectionManager;
use std::borrow::Cow;

pub struct Handler;

impl Handler {
    pub fn handle(request: Request) -> Result<HandlerResult, MainErrorKind> {
        let mut connection_manager: ConnectionManager = ConnectionManager::new();
        connection_manager.establish_connection()?;

        match ApplicationUserLogInTokenBaseRepository::get_by_device_id_and_value(&connection_manager, request.get_device_id(), request.get_token_value())? {
            Some(ref application_user_log_in_token) => {
                if !application_user_log_in_token.is_expired() {
                    let json_refresh_web_token: JsonRefreshWebToken<'_> = 
                        JsonRefreshWebToken::new(application_user_log_in_token.get_application_user_id(), Cow::Borrowed(application_user_log_in_token.get_device_id()));

                    connection_manager.begin_transaction()?;
                    match ApplicationUserLogInTokenBaseRepository::delete(&connection_manager, application_user_log_in_token) {
                        Ok(_) => {
                            match JsonRefreshWebTokenBaseRepository::create(&connection_manager, &json_refresh_web_token) {
                                Ok(_) => {
                                    connection_manager.commit_transaction()?;
                                    connection_manager.close_connection();

                                    return Ok(HandlerResult::new(SerializationFormResolver::serialize(&JsonAccessWebToken::new_from_json_refresh_web_token(&json_refresh_web_token))));
                                },
                                Err(diesel_error_kind) => {
                                    connection_manager.rollback_transaction()?;
        
                                    return Err(diesel_error_kind)?;
                                }
                            };
                        },
                        Err(diesel_error_kind) => {
                            connection_manager.rollback_transaction()?;

                            return Err(diesel_error_kind)?;
                        }
                    };
                } else {
                    return Err(EntityErrorKind::ApplicationUserLogInTokenErrorKind(ApplicationUserLogInTokenErrorKind::AlreadyExpired))?;
                }
            },
            None => {
                return Err(EntityErrorKind::ApplicationUserLogInTokenErrorKind(ApplicationUserLogInTokenErrorKind::NotFound))?;
            }
        };
    }
}