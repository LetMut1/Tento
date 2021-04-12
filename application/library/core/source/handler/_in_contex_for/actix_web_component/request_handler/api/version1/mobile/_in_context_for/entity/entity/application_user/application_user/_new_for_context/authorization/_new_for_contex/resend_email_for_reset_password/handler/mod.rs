use crate::dto::request_parameters::_in_context_for::actix_web_component::request_handler::api::version1::mobile::_in_context_for::entity::entity::application_user::application_user::_new_for_context::authorization::_new_for_context::resend_email_for_reset_password::request::Request;
use crate::entity::entity::application_user::application_user::core::email::Email;
use crate::error::main_error_kind::core::_in_context_for::entity::_new_for_context::entity_error_kind::core::_in_context_for::entity::reset_password_token::_new_for_context::reset_password_token::ResetPasswordTokenErrorKind;
use crate::error::main_error_kind::core::_in_context_for::entity::_new_for_context::entity_error_kind::entity_error_kind::EntityErrorKind;
use crate::error::main_error_kind::main_error_kind::MainErrorKind;
use crate::repository::_in_context_for::entity::entity::reset_password_token::_new_for_context::postgresql::base_repository::BaseRepository as ResetPasswordTokenBaseRepository;
use crate::service::_in_context_for::entity::entity::reset_password_token::_new_for_context::base_sender::BaseSender;
use crate::utility::_in_context_for::diesel_component::_new_for_context::postgresql::connection_manager::ConnectionManager;

pub struct Handler;

impl<'outer> Handler {
    pub fn handle(request: Request) -> Result<(), MainErrorKind> {
        let application_user_email: Email = Email::new(request.application_user_email);

        let mut connection_manager: ConnectionManager = ConnectionManager::new();
        connection_manager.establish_connection()?;

        if let Some(reset_password_token) = ResetPasswordTokenBaseRepository::get_by_application_user_email(&connection_manager, &application_user_email)? {
            connection_manager.close_connection();

            BaseSender::send_by_email(&reset_password_token, &application_user_email)?;
    
            return Ok(());
        }

        return Err(EntityErrorKind::ResetPasswordTokenErrorKind(ResetPasswordTokenErrorKind::NotFound))?;
    }
}
    // TODO Переделать емэйл на АппликешнЮзерАйди. Посмотреть, можно ли в эти методы класть сразу айдишник ( и уйти от лишних проверок)