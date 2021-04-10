use crate::dto::request_parameters::_in_context_for::actix_web_component::request_handler::api::version1::mobile::_in_context_for::entity::entity::application_user::application_user::_new_for_context::authorization::_new_for_context::pre_log_in::request::Request;
use crate::dto::response_parameters::_in_context_for::handler::_in_context_for::actix_web_component::request_handler::api::version1::mobile::_in_context_for::entity::entity::application_user::application_user::_new_for_context::authorization::_new_for_context::pre_log_in::handler::_new_for_context::result::Result as HandlerResult;
use crate::entity::core::uuid_v4::UuidV4;
use crate::entity::entity::application_user_log_in_token::application_user_log_in_token::ApplicationUserLogInToken;
use crate::entity::entity::application_user::application_user::core::email::Email;
use crate::entity::entity::application_user::application_user::core::password::Password;
use crate::error::main_error_kind::core::_in_context_for::entity::_new_for_context::entity_error_kind::core::_in_context_for::entity::application_user_log_in_token::_new_for_context::application_user_log_in_token::ApplicationUserLogInTokenErrorKind;
use crate::error::main_error_kind::core::_in_context_for::entity::_new_for_context::entity_error_kind::core::_in_context_for::entity::application_user::application_user::_new_for_context::application_user_error_kind::ApplicationUserErrorKind;
use crate::error::main_error_kind::core::_in_context_for::entity::_new_for_context::entity_error_kind::entity_error_kind::EntityErrorKind;
use crate::error::main_error_kind::main_error_kind::MainErrorKind;
use crate::repository::_in_context_for::entity::entity::application_user_log_in_token::_new_for_context::postgresql::base_repository::BaseRepository as ApplicationUserLogInTokenBaseRepository;
use crate::repository::_in_context_for::entity::entity::application_user::application_user::_new_for_context::postgresql::base_repository::BaseRepository as ApplicationUserBaseRepository;
use crate::service::_in_context_for::entity::entity::application_user_log_in_token::_new_for_context::base_sender::BaseSender;
use crate::utility::_in_context_for::diesel_component::_new_for_context::postgresql::connection_manager::ConnectionManager;
use crate::utility::_in_context_for::entity::entity::application_user::application_user::core::password::_new_for_context::password_encoder::PasswordEncoder;

pub struct Handler;

impl Handler {
    pub fn handle(request: Request) -> Result<HandlerResult, MainErrorKind> {
        let mut connection_manager: ConnectionManager = ConnectionManager::new();
        connection_manager.establish_connection()?;

        if let Some(application_user) = ApplicationUserBaseRepository::get_by_email(&connection_manager, &Email::new(request.application_user_email))? {
            if PasswordEncoder::is_valid(&Password::new(request.application_user_password), application_user.get_passord_hash()) {
                if !ApplicationUserLogInTokenBaseRepository::is_exist_by_application_user_id(&connection_manager, application_user.get_id())? {
                    let application_user_log_in_token: ApplicationUserLogInToken<'_> = 
                    ApplicationUserLogInToken::new(&application_user, UuidV4::new_from_str(request.application_user_log_in_token_device_id.as_str())?);

                    ApplicationUserLogInTokenBaseRepository::create(&connection_manager, &application_user_log_in_token)?;

                    connection_manager.close_connection();

                    BaseSender::send_by_email(&application_user_log_in_token, application_user.get_email())?;

                    return Ok(HandlerResult::new(application_user.get_id().get_value().to_string()));
                }
                
                return Err(EntityErrorKind::ApplicationUserLogInTokenErrorKind(ApplicationUserLogInTokenErrorKind::AlreadyExist))?;
            }
            
            return Err(EntityErrorKind::ApplicationUserErrorKind(ApplicationUserErrorKind::WrongPassword))?;
            
        }

        return Err(EntityErrorKind::ApplicationUserErrorKind(ApplicationUserErrorKind::NotFound))?;
    }
}