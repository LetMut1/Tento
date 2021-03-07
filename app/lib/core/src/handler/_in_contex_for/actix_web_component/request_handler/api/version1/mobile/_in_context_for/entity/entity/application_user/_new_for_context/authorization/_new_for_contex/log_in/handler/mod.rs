use crate::diesel_component::model::_in_context_for::entity::entity::application_user::_new_for_context::existing::Existing as ApplicationUserExisting;
use crate::diesel_component::model::_in_context_for::entity::entity::json_web_token::json_refresh_web_token::_new_for_context::new::New as JsonRefreshWebTokenNew;
use crate::dto::_in_context_for::actix_web_component::request_handler::api::version1::mobile::_in_context_for::entity::entity::application_user::_new_for_context::authorization::_new_for_context::log_in::request::Request;
use crate::dto::_in_context_for::handler::_in_context_for::actix_web_component::request_handler::api::version1::mobile::_in_context_for::entity::entity::application_user::_new_for_context::authorization::_new_for_context::log_in::handler::_new_for_context::result::Result as HandlerResult;
use crate::entity::entity::application_user::application_user::ApplicationUser;
use crate::entity::entity::json_web_token::json_access_web_token::json_access_web_token::JsonAccessWebToken;
use crate::entity::entity::json_web_token::json_refresh_web_token::json_refresh_web_token::JsonRefreshWebToken;
use crate::error::main_error_kind::core::_in_context_for::entity::_new_for_context::entity_error_kind::core::_in_context_for::entity::application_user::_new_for_context::application_user_error_kind::ApplicationUserErrorKind;
use crate::error::main_error_kind::core::_in_context_for::entity::_new_for_context::entity_error_kind::entity_error_kind::EntityErrorKind;
use crate::error::main_error_kind::main_error_kind::MainErrorKind;
use crate::repository::_in_context_for::entity::entity::application_user::_new_for_context::base_repository::BaseRepository as ApplicationUserBaseRepository;
use crate::repository::_in_context_for::entity::entity::json_web_token::json_refresh_web_token::_new_for_context::base_repository::BaseRepository as JsonRefreshWebTokenBaseRepository;
use crate::service::_in_context_for::entity::entity::json_web_token::json_access_web_token::_new_for_context::serialization_form_resolver::SerializationFormResolver;
use crate::utility::_in_context_for::entity::entity::application_user::core::password::_new_for_context::password_encoder::PasswordEncoder;
use crate::utility::_in_context_for::repository::_new_for_context::pg_connection_manager::PGConnectionManager;

pub struct Handler;

impl<'outer> Handler {
    pub fn handle(request: &'outer Request) -> Result<HandlerResult, MainErrorKind> {
        let mut pg_connection_manager: PGConnectionManager = PGConnectionManager::new();
        pg_connection_manager.establish_connection()?;
        let application_user_existing: ApplicationUserExisting = 
            ApplicationUserBaseRepository::get_by_email(pg_connection_manager.get_connection(), request.get_email())?;
        let application_user: ApplicationUser<'_> = ApplicationUser::new_from_model(&application_user_existing);
        if  PasswordEncoder::is_valid(request.get_password(), application_user.get_passord_hash().get_value()) {
            if application_user.is_confirmed() {
                let json_refresh_web_token: JsonRefreshWebToken<'_> = 
                    JsonRefreshWebToken::new(&application_user, request.get_device_id());   
                let json_refresh_web_token_new: JsonRefreshWebTokenNew<'_> = JsonRefreshWebTokenNew::new_from_entity(&json_refresh_web_token);
                JsonRefreshWebTokenBaseRepository::save(pg_connection_manager.get_connection(), &json_refresh_web_token_new)?;
                pg_connection_manager.close_connection(); 
                let json_access_web_token: JsonAccessWebToken<'_> = JsonAccessWebToken::new_from_json_refresh_web_token(&json_refresh_web_token);
                
                return Ok(HandlerResult::new(SerializationFormResolver::serialize(&json_access_web_token)));
            } else {
                pg_connection_manager.close_connection();

                return Err(EntityErrorKind::ApplicationUserErrorKind(ApplicationUserErrorKind::NotConfirmed))?;
            }
        } else {
            pg_connection_manager.close_connection();

            return Err(EntityErrorKind::ApplicationUserErrorKind(ApplicationUserErrorKind::WrongPassword))?;
        }
    }
}