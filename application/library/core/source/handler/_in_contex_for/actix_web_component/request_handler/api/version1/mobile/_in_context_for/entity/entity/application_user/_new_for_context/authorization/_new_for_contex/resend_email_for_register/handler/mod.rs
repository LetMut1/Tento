use crate::data_transfer_object::request_parameters::_in_context_for::actix_web_component::request_handler::api::version1::mobile::_in_context_for::entity::entity::application_user::_new_for_context::authorization::_new_for_context::resend_email_for_register::request::Request;
use crate::entity::entity::application_user_registration_confirmation_token::application_user_registration_confirmation_token::ApplicationUserRegistrationConfirmationToken;
use crate::entity::entity::application_user::core::email::Email;
use crate::error::main_error_kind::core::_in_context_for::entity::_new_for_context::entity_error_kind::core::_in_context_for::entity::pre_confirmed_application_user::_new_for_context::pre_confirmed_application_user_error_kind::PreConfirmedApplicationUserErrorKind;
use crate::error::main_error_kind::core::_in_context_for::entity::_new_for_context::entity_error_kind::entity_error_kind::EntityErrorKind;
use crate::error::main_error_kind::main_error_kind::MainErrorKind;
use crate::repository::_in_context_for::entity::entity::application_user_registration_confirmation_token::_new_for_context::postgresql::base_repository::BaseRepository as ApplicationUserRegistrationConfirmationTokenBaseRepository;
use crate::repository::_in_context_for::entity::entity::pre_confirmed_application_user::_new_for_context::postgresql::base_repository::BaseRepository as PreConfirmedApplicationUserBaseRepository;
use crate::service::_in_context_for::entity::entity::application_user::_new_for_context::email_sender::EmailSender;
use crate::utility::resource_connection::postgresql::connection_manager::ConnectionManager;

pub struct Handler;

impl Handler {
    pub fn handle(request: Request) -> Result<(), MainErrorKind> { // TODO сделать На Редисе механизм для невозможности почстоянно отравки емэйла. (Сохранять, если отправлено, и проверять, что отпрпавили. удалять по времени)
        let mut connection_manager: ConnectionManager = ConnectionManager::new();
        connection_manager.establish_connection()?;

        if let Some(pre_confirmed_application_user) = PreConfirmedApplicationUserBaseRepository::get_by_email(&connection_manager, &Email::new(request.application_user_email))? {
            let mut application_user_registration_confirmation_token: ApplicationUserRegistrationConfirmationToken<'_>;

            match ApplicationUserRegistrationConfirmationTokenBaseRepository::get_by_pre_confirmed_application_user_id(&connection_manager, pre_confirmed_application_user.get_id())? {
                Some(existing_application_user_registration_confirmation_token) => {
                    application_user_registration_confirmation_token = existing_application_user_registration_confirmation_token;
                    application_user_registration_confirmation_token.refresh();

                    ApplicationUserRegistrationConfirmationTokenBaseRepository::update(&connection_manager, &application_user_registration_confirmation_token)?;
                },
                None => {
                    application_user_registration_confirmation_token = ApplicationUserRegistrationConfirmationToken::new(&pre_confirmed_application_user);

                    ApplicationUserRegistrationConfirmationTokenBaseRepository::create(&connection_manager, &application_user_registration_confirmation_token)?;
                }
            }

            connection_manager.close_connection();
            
            EmailSender::send_application_user_registration_confirmation_token(&application_user_registration_confirmation_token)?;

            return Ok(());
        }

        return Err(EntityErrorKind::PreConfirmedApplicationUserErrorKind(PreConfirmedApplicationUserErrorKind::NotFound))?;
    }
}