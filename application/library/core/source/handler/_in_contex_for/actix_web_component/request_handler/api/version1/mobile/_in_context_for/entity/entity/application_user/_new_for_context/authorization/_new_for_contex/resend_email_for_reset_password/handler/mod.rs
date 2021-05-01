use crate::data_transfer_object::request_parameters::_in_context_for::actix_web_component::request_handler::api::version1::mobile::_in_context_for::entity::entity::application_user::_new_for_context::authorization::_new_for_context::resend_email_for_reset_password::request::Request;
use crate::entity::core::uuid_v4::UuidV4;
use crate::error::main_error_kind::core::entity_error_kind::core::_in_context_for::entity::entity::application_user_reset_password_token::_new_for_context::application_user_reset_password_token_error_kind::ApplicationUserResetPasswordTokenErrorKind;
use crate::error::main_error_kind::core::entity_error_kind::entity_error_kind::EntityErrorKind;
use crate::error::main_error_kind::main_error_kind::MainErrorKind;
use crate::repository::_in_context_for::entity::entity::application_user_reset_password_token::_new_for_context::redis::base_repository::BaseRepository as ApplicationUserResetPasswordTokenBaseRepository;
use crate::service::_in_context_for::entity::entity::application_user::_new_for_context::email_sender::EmailSender;
use crate::utility::resource_connection::redis::connection_manager::ConnectionManager;

pub struct Handler;

impl<'outer> Handler {
    pub fn handle(request: Request) -> Result<(), MainErrorKind> {
        let application_user_id: UuidV4 = UuidV4::new_from_string(request.application_user_id)?;

        let mut connection_manager: ConnectionManager = ConnectionManager::new();
        connection_manager.establish_connection()?;

        if let Some(application_user_reset_password_token) = ApplicationUserResetPasswordTokenBaseRepository::get_by_application_user_id(&mut connection_manager, &application_user_id)? {
            connection_manager.close_connection();

            EmailSender::send_application_user_reset_password_token(&application_user_reset_password_token)?;
    
            return Ok(());
        }

        return Err(MainErrorKind::EntityErrorKind(EntityErrorKind::ApplicationUserResetPasswordTokenErrorKind(ApplicationUserResetPasswordTokenErrorKind::NotFound)));
    }
}