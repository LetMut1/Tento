use crate::dto::request_parameters::_in_context_for::actix_web_component::request_handler::api::version1::mobile::_in_context_for::entity::entity::application_user::application_user::_new_for_context::authorization::_new_for_context::resend_email_for_register::request::Request;
use crate::entity::core::uuid_v4::UuidV4;
use crate::error::main_error_kind::core::_in_context_for::entity::_new_for_context::entity_error_kind::core::_in_context_for::entity::application_user_registration_confirmation_token::_new_for_context::application_user_registration_confirmation_token_error_kind::ApplicationUserRegistrationConfirmationTokenErrorKind;
use crate::error::main_error_kind::core::_in_context_for::entity::_new_for_context::entity_error_kind::entity_error_kind::EntityErrorKind;
use crate::error::main_error_kind::main_error_kind::MainErrorKind;
use crate::repository::_in_context_for::entity::entity::application_user_registration_confirmation_token::_new_for_context::postgresql::base_repository::BaseRepository as ApplicationUserRegistrationConfirmationTokenBaseRepository;
use crate::service::_in_context_for::entity::entity::application_user_registration_confirmation_token::_new_for_context::base_sender::BaseSender;
use crate::utility::_in_context_for::diesel_component::_new_for_context::postgresql::connection_manager::ConnectionManager;

pub struct Handler;

impl Handler {
    pub fn handle(request: Request) -> Result<(), MainErrorKind> { // TODO сделать На Редисе механизм для невозможности почстоянно отравки емэйла. (Сохранять, если отправлено, и проверять, что отпрпавили. удалять по времени)
        let pre_confirmed_application_user_id: UuidV4 = UuidV4::new_from_string(request.pre_confirmed_application_user_id)?;

        let mut connection_manager: ConnectionManager = ConnectionManager::new();
        connection_manager.establish_connection()?;

        if let Some(application_user_registration_confirmation_token) = 
        ApplicationUserRegistrationConfirmationTokenBaseRepository::get_by_pre_confirmed_application_user_id(&connection_manager, &pre_confirmed_application_user_id)? 
        {
            connection_manager.close_connection();
            
            BaseSender::send_by_email(&application_user_registration_confirmation_token)?;

            return Ok(());
        }

        return Err(EntityErrorKind::ApplicationUserRegistrationConfirmationTokenErrorKind(ApplicationUserRegistrationConfirmationTokenErrorKind::NotFound))?;
    }
}