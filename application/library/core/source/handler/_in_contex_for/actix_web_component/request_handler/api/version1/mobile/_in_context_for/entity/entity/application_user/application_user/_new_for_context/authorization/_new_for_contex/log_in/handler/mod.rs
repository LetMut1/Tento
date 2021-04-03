use crate::dto::_in_context_for::actix_web_component::request_handler::api::version1::mobile::_in_context_for::entity::entity::application_user::application_user::_new_for_context::authorization::_new_for_context::log_in::request::Request;
use crate::dto::_in_context_for::handler::_in_context_for::actix_web_component::request_handler::api::version1::mobile::_in_context_for::entity::entity::application_user::application_user::_new_for_context::authorization::_new_for_context::log_in::handler::_new_for_context::result::Result as HandlerResult;
use crate::entity::core::uuid_v4::UuidV4;
use crate::entity::entity::json_web_token::json_access_web_token::json_access_web_token::JsonAccessWebToken;
use crate::entity::entity::json_web_token::json_refresh_web_token::json_refresh_web_token::JsonRefreshWebToken;
use crate::error::main_error_kind::core::_in_context_for::entity::_new_for_context::entity_error_kind::core::_in_context_for::entity::application_user_log_in_token::_new_for_context::application_user_log_in_token::ApplicationUserLogInTokenErrorKind;
use crate::error::main_error_kind::core::_in_context_for::entity::_new_for_context::entity_error_kind::entity_error_kind::EntityErrorKind;
use crate::error::main_error_kind::main_error_kind::MainErrorKind;
use crate::repository::_in_context_for::entity::entity::application_user_log_in_token::_new_for_context::postgresql::base_repository::BaseRepository as ApplicationUserLogInTokenBaseRepository;
use crate::repository::_in_context_for::entity::entity::json_web_token::json_refresh_web_token::_new_for_context::postgresql::base_repository::BaseRepository as JsonRefreshWebTokenBaseRepository;
use crate::service::_in_context_for::entity::entity::json_web_token::json_access_web_token::_new_for_context::serialization_form_resolver::SerializationFormResolver;
use crate::service::_in_context_for::entity::entity::json_web_token::json_refresh_web_token::_new_for_context::encoder::Encoder;
use crate::utility::_in_context_for::diesel_component::_new_for_context::postgresql::connection_manager::ConnectionManager;
use std::borrow::Cow;

pub struct Handler;

impl Handler {
    pub fn handle(request: Request) -> Result<HandlerResult, MainErrorKind> {
        let mut connection_manager: ConnectionManager = ConnectionManager::new();
        connection_manager.establish_connection()?;

        if let Some(ref application_user_log_in_token) = ApplicationUserLogInTokenBaseRepository::get_by_application_user_id_and_device_id(
            &connection_manager, &UuidV4::new_from_str(request.application_user_id.as_str())?, &UuidV4::new_from_str(request.application_user_log_in_token_device_id.as_str())?
        )?
        {
            if application_user_log_in_token.get_value().get_value() == request.application_user_log_in_token_value {
                if !application_user_log_in_token.is_expired() {
                    let json_refresh_web_token: JsonRefreshWebToken<'_> = 
                    JsonRefreshWebToken::new(application_user_log_in_token.get_application_user_id(), Cow::Borrowed(application_user_log_in_token.get_device_id()));

                    connection_manager.begin_transaction()?;

                    if let Err(diesel_error) = ApplicationUserLogInTokenBaseRepository::delete(&connection_manager, application_user_log_in_token) { 
                        connection_manager.rollback_transaction()?;

                        return Err(diesel_error)?;
                        
                    }

                    if let Err(diesel_error) = JsonRefreshWebTokenBaseRepository::create(&connection_manager, &json_refresh_web_token) {
                        connection_manager.rollback_transaction()?;

                        return Err(diesel_error)?;
                    }

                    connection_manager.commit_transaction()?;
                    connection_manager.close_connection();

                    // TODO // TODO // TODO // TODO // TODO Если уже есть в JRWT (юзерайди+девайсайди) (повторный логин, по сути, то делаем РАЗЛОГИН старого токена), значит сделать перелогин, то есть, инвалидировать предыдущие значения
                    // удалить имеющийся JRWT, записать его аксесс в блэклист

                    return Ok(
                        HandlerResult::new(
                            SerializationFormResolver::serialize(&JsonAccessWebToken::new_from_json_refresh_web_token(&json_refresh_web_token)),
                            Encoder::encode(&json_refresh_web_token)
                        )
                    );
                }
                
                return Err(EntityErrorKind::ApplicationUserLogInTokenErrorKind(ApplicationUserLogInTokenErrorKind::AlreadyExpired))?;
            }
            
            return Err(EntityErrorKind::ApplicationUserLogInTokenErrorKind(ApplicationUserLogInTokenErrorKind::InvalidValue))?;
        }

        return Err(EntityErrorKind::ApplicationUserLogInTokenErrorKind(ApplicationUserLogInTokenErrorKind::NotFound))?;
    }
}