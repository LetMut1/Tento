use crate::dto::_in_context_for::actix_web_component::request_handler::api::version1::mobile::_in_context_for::entity::entity::application_user::_new_for_context::authorization::_new_for_context::log_in::request::Request;
use crate::dto::_in_context_for::handler::_in_context_for::actix_web_component::request_handler::api::version1::mobile::_in_context_for::entity::entity::application_user::_new_for_context::authorization::_new_for_context::log_in::handler::_new_for_context::handler_result::HandlerResult;
use crate::entity::entity::application_user::application_user::ApplicationUser;
use crate::entity::entity::json_web_token::json_access_web_token::json_access_web_token::JsonAccessWebToken;
use crate::entity::entity::json_web_token::json_refresh_web_token::core::device_id::DeviceId;
use crate::entity::entity::json_web_token::json_refresh_web_token::json_refresh_web_token::JsonRefreshWebToken;
use crate::error::main_error_kind::core::_in_context_for::entity::_new_for_context::entity_error_kind::core::_in_context_for::entity::application_user::_new_for_context::application_user_error_kind::ApplicationUserErrorKind;
use crate::error::main_error_kind::core::_in_context_for::entity::_new_for_context::entity_error_kind::entity_error_kind::EntityErrorKind;
use crate::error::main_error_kind::main_error_kind::MainErrorKind;
use crate::repository::_in_context_for::entity::entity::application_user::_new_for_context::base_repository::BaseRepository as ApplicationUserBaseRepository;
use crate::repository::_in_context_for::entity::entity::json_web_token::json_refresh_web_token::_new_for_context::base_repository::BaseRepository as JsonRefreshWebTokenBaseRepository;
use crate::service::_in_context_for::entity::entity::json_web_token::json_access_web_token::_new_for_context::serialization_form_resolver::SerializationFormResolver;
use crate::utility::_in_context_for::diesel_component::_new_for_context::postgresql::connection_manager::ConnectionManager;
use crate::utility::_in_context_for::entity::entity::application_user::core::password::_new_for_context::password_encoder::PasswordEncoder;

pub struct Handler;

impl Handler {
    pub fn handle(request: Request) -> Result<HandlerResult, MainErrorKind> {
        let mut connection_manager: ConnectionManager = ConnectionManager::new();
        connection_manager.establish_connection()?;

        match ApplicationUserBaseRepository::get_by_email(&connection_manager, request.get_email())? {
            Some(value) => {
                let application_user: ApplicationUser = ApplicationUser::new_from_model(value);

                if  PasswordEncoder::is_valid(request.get_password(), application_user.get_passord_hash().get_value()) {
                    if application_user.is_confirmed() {
                        let json_refresh_web_token: JsonRefreshWebToken<'_> = JsonRefreshWebToken::new(&application_user, DeviceId::new(request.device_id));

                        JsonRefreshWebTokenBaseRepository::create(&connection_manager, &json_refresh_web_token)?;

                        connection_manager.close_connection(); 
                        
                        let json_access_web_token: JsonAccessWebToken<'_> = JsonAccessWebToken::new_from_json_refresh_web_token(&json_refresh_web_token);
                        
                        return Ok(HandlerResult::new(SerializationFormResolver::serialize(&json_access_web_token)));
                    } else {
                        connection_manager.close_connection();

                        return Err(EntityErrorKind::ApplicationUserErrorKind(ApplicationUserErrorKind::NotConfirmed))?;
                    }
                } else {
                    connection_manager.close_connection();

                    return Err(EntityErrorKind::ApplicationUserErrorKind(ApplicationUserErrorKind::WrongPassword))?;
                }
            },
            None => {
                connection_manager.close_connection();
                
                return Err(EntityErrorKind::ApplicationUserErrorKind(ApplicationUserErrorKind::NotFound))?;
            }
        };
    }
}