use crate::dto::_in_context_for::actix_web_component::request_handler::api::version1::mobile::_in_context_for::entity::entity::application_user::_new_for_context::authorization::_new_for_context::register::request::Request;
use crate::dto::_in_context_for::handler::_in_context_for::actix_web_component::request_handler::api::version1::mobile::_in_context_for::entity::entity::application_user::_new_for_context::authorization::_new_for_context::register::handler::_new_for_context::handler_result::HandlerResult;
use crate::entity::entity::application_user_registration_confirmation_token::application_user_registration_confirmation_token::ApplicationUserRegistrationConfirmationToken;
use crate::entity::entity::application_user::application_user::ApplicationUser;
use crate::error::main_error_kind::core::_in_context_for::entity::_new_for_context::entity_error_kind::core::_in_context_for::entity::application_user::_new_for_context::application_user_error_kind::ApplicationUserErrorKind;
use crate::error::main_error_kind::core::_in_context_for::entity::_new_for_context::entity_error_kind::entity_error_kind::EntityErrorKind;
use crate::error::main_error_kind::main_error_kind::MainErrorKind;
use crate::repository::_in_context_for::entity::entity::application_user_registration_confirmation_token::_new_for_context::base_repository::BaseRepository as ApplicationUserRegistrationConfirmationTokenBaseRepository;
use crate::repository::_in_context_for::entity::entity::application_user::_new_for_context::base_repository::BaseRepository as ApplicationUserBaseRepository;
use crate::resourse_model::_in_context_for::entity::entity::application_user_registration_confirmation_token::_new_for_context::new::New as ApplicationUserRegistrationConfirmationTokenNew;
use crate::resourse_model::_in_context_for::entity::entity::application_user::_new_for_context::new::New as ApplicationUserNew;
use crate::service::_in_context_for::entity::entity::application_user_registration_confirmation_token::_new_for_context::base_sender::BaseSender;
use crate::utility::_in_context_for::diesel_component::_new_for_context::postgresql::connection_manager::ConnectionManager;
use crate::utility::_in_context_for::entity::entity::application_user::core::email::_new_for_context::email_simple_validator::EmailSimpleValidator;

pub struct Handler;

impl<'outer> Handler {
    pub fn handle(request: &'outer Request) -> Result<HandlerResult, MainErrorKind> {
        if EmailSimpleValidator::is_valid(request.get_email()) {
            let mut connection_manager: ConnectionManager = ConnectionManager::new();
            connection_manager.establish_connection()?;

            if !ApplicationUserBaseRepository::is_exist_by_nickanme(&connection_manager, request.get_nickname())? 
                && !ApplicationUserBaseRepository::is_exist_by_email(&connection_manager, request.get_email())?
            {
                let application_user: ApplicationUser<'_> = ApplicationUser::new(request.get_email(), request.get_nickname(), request.get_password());  

                let application_user_registration_confirmation_token: ApplicationUserRegistrationConfirmationToken<'_> = ApplicationUserRegistrationConfirmationToken::new(&application_user);
                
                connection_manager.begin_transaction()?;
                match ApplicationUserBaseRepository::save(&connection_manager, &ApplicationUserNew::new(&application_user)) {
                    Ok(_) => {
                        match ApplicationUserRegistrationConfirmationTokenBaseRepository::save(&connection_manager, &ApplicationUserRegistrationConfirmationTokenNew::new(&application_user_registration_confirmation_token)) {
                            Ok(_) => {
                                connection_manager.commit_transaction()?;
                                connection_manager.close_connection();

                                BaseSender::send_by_email(&application_user_registration_confirmation_token, application_user.get_email())?;
                            },
                            Err(value) => {
                                connection_manager.rollback_transaction()?;

                                return Err(value)?;
                            }
                        };
                    },
                    Err(value) => {
                        connection_manager.rollback_transaction()?;

                        return Err(value)?;
                    }
                };
            } else {
                connection_manager.close_connection();

                return Err(EntityErrorKind::ApplicationUserErrorKind(ApplicationUserErrorKind::AlreadyExist))?;
            }
            
            return Ok(HandlerResult::new(true));
        } else {
            return Err(EntityErrorKind::ApplicationUserErrorKind(ApplicationUserErrorKind::InvalidEmail))?;
        }
    }
}