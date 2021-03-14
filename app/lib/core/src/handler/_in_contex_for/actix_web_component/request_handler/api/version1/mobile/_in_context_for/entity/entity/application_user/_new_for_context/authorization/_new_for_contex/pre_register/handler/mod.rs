use crate::dto::_in_context_for::actix_web_component::request_handler::api::version1::mobile::_in_context_for::entity::entity::application_user::_new_for_context::authorization::_new_for_context::pre_register::request::Request;
use crate::dto::_in_context_for::handler::_in_context_for::actix_web_component::request_handler::api::version1::mobile::_in_context_for::entity::entity::application_user::_new_for_context::authorization::_new_for_context::pre_register::handler::_new_for_context::handler_result::HandlerResult;
use crate::entity::entity::application_user::core::email::Email;
use crate::entity::entity::pre_confirmed_application_user::pre_confirmed_application_user::PreConfirmedApplicationUser;
use crate::entity::entity::pre_registered_application_user_registration_confirmation_token::pre_registered_application_user_registration_confirmation_token::PreRegisteredApplicationUserRegistrationConfirmationToken;
use crate::error::main_error_kind::core::_in_context_for::entity::_new_for_context::entity_error_kind::core::_in_context_for::entity::application_user::_new_for_context::application_user_error_kind::ApplicationUserErrorKind;
use crate::error::main_error_kind::core::_in_context_for::entity::_new_for_context::entity_error_kind::core::_in_context_for::entity::pre_confirmed_application_user::_new_for_context::pre_confirmed_application_user_error_kind::PreConfirmedApplicationUserErrorKind;
use crate::error::main_error_kind::core::_in_context_for::entity::_new_for_context::entity_error_kind::entity_error_kind::EntityErrorKind;
use crate::error::main_error_kind::main_error_kind::MainErrorKind;
use crate::repository::_in_context_for::entity::entity::application_user::_new_for_context::postgresql::base_repository::BaseRepository as ApplicationUserBaseRepository;
use crate::repository::_in_context_for::entity::entity::pre_confirmed_application_user::_new_for_context::postgresql::base_repository::BaseRepository as PreConfirmedApplicationUserBaseRepository;
use crate::repository::_in_context_for::entity::entity::pre_registered_application_user_registration_confirmation_token::_new_for_context::postgresql::base_repository::BaseRepository as PreRegisteredApplicationUserRegistrationConfirmationTokenBaseRepository;
use crate::service::_in_context_for::entity::entity::pre_registered_application_user_registration_confirmation_token::_new_for_context::base_sender::BaseSender;
use crate::utility::_in_context_for::diesel_component::_new_for_context::postgresql::connection_manager::ConnectionManager;
use crate::utility::_in_context_for::entity::entity::application_user::core::email::_new_for_context::email_simple_validator::EmailSimpleValidator;

pub struct Handler;

impl Handler {
    pub fn handle(request: Request) -> Result<HandlerResult, MainErrorKind> {
        if EmailSimpleValidator::is_valid(request.get_email()) {
            let mut connection_manager: ConnectionManager = ConnectionManager::new();
            connection_manager.establish_connection()?;

            if !PreConfirmedApplicationUserBaseRepository::is_exist_by_email(&connection_manager, request.get_email())? {
                if !ApplicationUserBaseRepository::is_exist_by_email(&connection_manager, request.get_email())? {
                    let pre_confirmed_application_user: PreConfirmedApplicationUser = PreConfirmedApplicationUser::new(Email::new(request.email));  

                    let pre_registered_application_user_registration_confirmation_token: PreRegisteredApplicationUserRegistrationConfirmationToken<'_> = 
                        PreRegisteredApplicationUserRegistrationConfirmationToken::new(&pre_confirmed_application_user);
                    
                    connection_manager.begin_transaction()?;
                    match PreConfirmedApplicationUserBaseRepository::create(&connection_manager, &pre_confirmed_application_user) {
                        Ok(_) => {
                            match PreRegisteredApplicationUserRegistrationConfirmationTokenBaseRepository::create(&connection_manager, &pre_registered_application_user_registration_confirmation_token) {
                                Ok(_) => {
                                    connection_manager.commit_transaction()?;
                                    connection_manager.close_connection();

                                    BaseSender::send_by_email(&pre_registered_application_user_registration_confirmation_token, pre_confirmed_application_user.get_email())?;

                                                
                                    return Ok(HandlerResult::new());
                                },
                                Err(diesel_error_kind) => {
                                    connection_manager.rollback_transaction()?;

                                    return Err(diesel_error_kind)?;
                                }
                            };
                        },
                        Err(diesel_error_kind) => {
                            connection_manager.rollback_transaction()?;

                            return Err(diesel_error_kind)?;
                        }
                    };
                } else {
                    connection_manager.close_connection();

                    return Err(EntityErrorKind::ApplicationUserErrorKind(ApplicationUserErrorKind::AlreadyExist))?;
                }
            } else {
                connection_manager.close_connection();

                return Err(EntityErrorKind::PreConfirmedApplicationUserErrorKind(PreConfirmedApplicationUserErrorKind::AlreadyExist))?;
            }
        } else {
            return Err(EntityErrorKind::ApplicationUserErrorKind(ApplicationUserErrorKind::InvalidEmail))?;
        }
    }
}