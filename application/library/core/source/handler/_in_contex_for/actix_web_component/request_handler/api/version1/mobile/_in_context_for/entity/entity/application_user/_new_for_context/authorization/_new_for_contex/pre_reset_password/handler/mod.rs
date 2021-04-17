use crate::dto::request_parameters::_in_context_for::actix_web_component::request_handler::api::version1::mobile::_in_context_for::entity::entity::application_user::_new_for_context::authorization::_new_for_context::pre_reset_password::request::Request;
use crate::dto::response_parameters::_in_context_for::handler::_in_context_for::actix_web_component::request_handler::api::version1::mobile::_in_context_for::entity::entity::application_user::_new_for_context::authorization::_new_for_context::pre_reset_password::handler::_new_for_context::result::Result as HandlerResult;
use crate::entity::entity::application_user::core::email::Email;
use crate::entity::entity::application_user_reset_password_token::application_user_reset_password_token::ApplicationUserResetPasswordToken;
use crate::error::main_error_kind::core::_in_context_for::entity::_new_for_context::entity_error_kind::core::_in_context_for::entity::application_user::_new_for_context::application_user_error_kind::ApplicationUserErrorKind;
use crate::error::main_error_kind::core::_in_context_for::entity::_new_for_context::entity_error_kind::entity_error_kind::EntityErrorKind;
use crate::error::main_error_kind::main_error_kind::MainErrorKind;
use crate::repository::_in_context_for::entity::entity::application_user::_new_for_context::postgresql::base_repository::BaseRepository as ApplicationUserBaseRepository;
use crate::repository::_in_context_for::entity::entity::application_user_reset_password_token::_new_for_context::postgresql::base_repository::BaseRepository as ApplicationUserResetPasswordTokenBaseRepository;
use crate::service::_in_context_for::entity::entity::application_user_reset_password_token::_new_for_context::base_sender::BaseSender;
use crate::utility::_in_context_for::diesel_component::_new_for_context::postgresql::connection_manager::ConnectionManager;

pub struct Handler;

impl<'outer> Handler {
    pub fn handle(request: Request) -> Result<HandlerResult, MainErrorKind> {
        let mut connection_manager: ConnectionManager = ConnectionManager::new();
        connection_manager.establish_connection()?;

        if let Some(application_user) = ApplicationUserBaseRepository::get_by_email(&connection_manager, &Email::new(request.application_user_email))? {
            let mut application_user_reset_password_token: ApplicationUserResetPasswordToken<'_>;

            match ApplicationUserResetPasswordTokenBaseRepository::get_by_application_user_id(&connection_manager, application_user.get_id())? {
                Some(existing_application_user_reset_password_token) => {
                    application_user_reset_password_token = existing_application_user_reset_password_token;
                    application_user_reset_password_token.refresh();

                    ApplicationUserResetPasswordTokenBaseRepository::update(&connection_manager, &application_user_reset_password_token)?;
                },
                None => {
                    application_user_reset_password_token = ApplicationUserResetPasswordToken::new(&application_user);

                    ApplicationUserResetPasswordTokenBaseRepository::create(&connection_manager, &application_user_reset_password_token)?;
                }
            }

            connection_manager.close_connection();

            BaseSender::send_by_email(&application_user_reset_password_token)?;

            return Ok(HandlerResult::new(application_user.get_id().get_value().to_string()));
        }

        return Err(EntityErrorKind::ApplicationUserErrorKind(ApplicationUserErrorKind::NotFound))?;
    }
}