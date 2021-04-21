use crate::data_transfer_object::request_parameters::_in_context_for::actix_web_component::request_handler::api::version1::mobile::_in_context_for::entity::entity::application_user::_new_for_context::authorization::_new_for_context::pre_register::request::Request;
use crate::entity::entity::application_user_registration_confirmation_token::application_user_registration_confirmation_token::ApplicationUserRegistrationConfirmationToken;
use crate::entity::entity::application_user::core::email::Email;
use crate::entity::entity::pre_confirmed_application_user::pre_confirmed_application_user::PreConfirmedApplicationUser;
use crate::error::main_error_kind::core::_in_context_for::entity::_new_for_context::entity_error_kind::core::_in_context_for::entity::application_user::_new_for_context::application_user_error_kind::ApplicationUserErrorKind;
use crate::error::main_error_kind::core::_in_context_for::entity::_new_for_context::entity_error_kind::core::_in_context_for::entity::pre_confirmed_application_user::_new_for_context::pre_confirmed_application_user_error_kind::PreConfirmedApplicationUserErrorKind;
use crate::error::main_error_kind::core::_in_context_for::entity::_new_for_context::entity_error_kind::entity_error_kind::EntityErrorKind;
use crate::error::main_error_kind::main_error_kind::MainErrorKind;
use crate::repository::_in_context_for::entity::entity::application_user_registration_confirmation_token::_new_for_context::postgresql::base_repository::BaseRepository as ApplicationUserRegistrationConfirmationTokenBaseRepository;
use crate::repository::_in_context_for::entity::entity::application_user::_new_for_context::postgresql::base_repository::BaseRepository as ApplicationUserBaseRepository;
use crate::repository::_in_context_for::entity::entity::pre_confirmed_application_user::_new_for_context::postgresql::base_repository::BaseRepository as PreConfirmedApplicationUserBaseRepository;
use crate::service::_in_context_for::entity::entity::application_user::_new_for_context::email_sender::EmailSender;
use crate::utility::_in_context_for::diesel_component::_new_for_context::postgresql::connection_manager::ConnectionManager;
use crate::utility::_in_context_for::entity::entity::application_user::core::email::_new_for_context::email_simple_validator::EmailSimpleValidator;

pub struct Handler;

impl Handler {
    pub fn handle(request: Request) -> Result<(), MainErrorKind> {
        let application_user_email: Email = Email::new(request.application_user_email);

        if EmailSimpleValidator::is_valid(&application_user_email) {
            let mut connection_manager: ConnectionManager = ConnectionManager::new();
            connection_manager.establish_connection()?;

            if !PreConfirmedApplicationUserBaseRepository::is_exist_by_email(&connection_manager, &application_user_email)? {
                if !ApplicationUserBaseRepository::is_exist_by_email(&connection_manager, &application_user_email)? {
                    let pre_confirmed_application_user: PreConfirmedApplicationUser = PreConfirmedApplicationUser::new(application_user_email);  

                    let application_user_registration_confirmation_token: ApplicationUserRegistrationConfirmationToken<'_> =
                    ApplicationUserRegistrationConfirmationToken::new(&pre_confirmed_application_user);

                    connection_manager.begin_transaction()?;

                    if let Err(diesel_error) = ApplicationUserRegistrationConfirmationTokenBaseRepository::create(&connection_manager, &application_user_registration_confirmation_token) {
                        connection_manager.rollback_transaction()?;

                        return Err(diesel_error)?;
                    }
                     

                    if let Err(diesel_error) = PreConfirmedApplicationUserBaseRepository::create(&connection_manager, &pre_confirmed_application_user) {
                        connection_manager.rollback_transaction()?;

                        return Err(diesel_error)?;
                    }
                    
                    connection_manager.commit_transaction()?;
                    connection_manager.close_connection();

                    EmailSender::send_application_user_registration_confirmation_token(&application_user_registration_confirmation_token)?;

                    return Ok(());
                }
                
                return Err(EntityErrorKind::ApplicationUserErrorKind(ApplicationUserErrorKind::AlreadyExist))?;
            }
            
            return Err(EntityErrorKind::PreConfirmedApplicationUserErrorKind(PreConfirmedApplicationUserErrorKind::AlreadyExist))?;
        }
        
        return Err(EntityErrorKind::ApplicationUserErrorKind(ApplicationUserErrorKind::InvalidEmail))?;
    }
}