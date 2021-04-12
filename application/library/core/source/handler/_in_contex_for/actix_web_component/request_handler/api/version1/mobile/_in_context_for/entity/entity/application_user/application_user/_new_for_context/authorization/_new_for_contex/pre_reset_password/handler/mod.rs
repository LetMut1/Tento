use crate::dto::request_parameters::_in_context_for::actix_web_component::request_handler::api::version1::mobile::_in_context_for::entity::entity::application_user::application_user::_new_for_context::authorization::_new_for_context::pre_reset_password::request::Request;
use crate::entity::entity::application_user::application_user::core::email::Email;
use crate::entity::entity::reset_password_token::reset_password_token::ResetPasswordToken;
use crate::error::main_error_kind::core::_in_context_for::entity::_new_for_context::entity_error_kind::core::_in_context_for::entity::application_user::application_user::_new_for_context::application_user_error_kind::ApplicationUserErrorKind;
use crate::error::main_error_kind::core::_in_context_for::entity::_new_for_context::entity_error_kind::entity_error_kind::EntityErrorKind;
use crate::error::main_error_kind::main_error_kind::MainErrorKind;
use crate::repository::_in_context_for::entity::entity::reset_password_token::_new_for_context::postgresql::base_repository::BaseRepository as ResetPasswordTokenBaseRepository;
use crate::repository::_in_context_for::entity::entity::application_user::application_user::_new_for_context::postgresql::base_repository::BaseRepository as ApplicationUserBaseRepository;
use crate::utility::_in_context_for::diesel_component::_new_for_context::postgresql::connection_manager::ConnectionManager;
use crate::service::_in_context_for::entity::entity::reset_password_token::_new_for_context::base_sender::BaseSender;

pub struct Handler;

impl<'outer> Handler {
    pub fn handle(request: Request) -> Result<(), MainErrorKind> {
        let application_user_email: Email = Email::new(request.application_user_email);

        let mut connection_manager: ConnectionManager = ConnectionManager::new();
        connection_manager.establish_connection()?;

        if ApplicationUserBaseRepository::is_exist_by_email(&connection_manager, &application_user_email)? {
            let mut reset_password_token: ResetPasswordToken<'_>;

            match ResetPasswordTokenBaseRepository::get_by_application_user_email(&connection_manager, &application_user_email)? {
                Some(existing_reset_password_token) => {
                    reset_password_token = existing_reset_password_token;
                    reset_password_token.refresh();

                    ResetPasswordTokenBaseRepository::update(&connection_manager, &reset_password_token)?;
                },
                None => {
                    reset_password_token = ResetPasswordToken::new(&application_user_email);

                    ResetPasswordTokenBaseRepository::create(&connection_manager, &reset_password_token)?;
                }
            }

            connection_manager.close_connection();

            BaseSender::send_by_email(&reset_password_token, &application_user_email)?;

            return Ok(());
        }

        return Err(EntityErrorKind::ApplicationUserErrorKind(ApplicationUserErrorKind::NotFound))?;
    }
}