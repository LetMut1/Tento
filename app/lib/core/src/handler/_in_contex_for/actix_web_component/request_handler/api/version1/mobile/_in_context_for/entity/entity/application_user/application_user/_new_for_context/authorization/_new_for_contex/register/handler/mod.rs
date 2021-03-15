use crate::dto::_in_context_for::actix_web_component::request_handler::api::version1::mobile::_in_context_for::entity::entity::application_user::application_user::_new_for_context::authorization::_new_for_context::register::request::Request;
use crate::dto::_in_context_for::handler::_in_context_for::actix_web_component::request_handler::api::version1::mobile::_in_context_for::entity::entity::application_user::application_user::_new_for_context::authorization::_new_for_context::register::handler::_new_for_context::handler_result::HandlerResult;
use crate::entity::entity::application_user::application_user::application_user::ApplicationUser;
use crate::entity::entity::application_user::application_user::core::nickname::Nickname;
use crate::entity::entity::application_user::application_user::core::password::Password;
use crate::error::main_error_kind::core::_in_context_for::entity::_new_for_context::entity_error_kind::core::_in_context_for::entity::application_user::application_user::_new_for_context::application_user_error_kind::ApplicationUserErrorKind;
use crate::error::main_error_kind::core::_in_context_for::entity::_new_for_context::entity_error_kind::core::_in_context_for::entity::application_user::pre_confirmed_application_user::_new_for_context::pre_confirmed_application_user_error_kind::PreConfirmedApplicationUserErrorKind;
use crate::error::main_error_kind::core::_in_context_for::entity::_new_for_context::entity_error_kind::core::_in_context_for::entity::application_user_registration_confirmation_token::_new_for_context::application_user_registration_confirmation_token_error_kind::ApplicationUserRegistrationConfirmationTokenErrorKind;
use crate::error::main_error_kind::core::_in_context_for::entity::_new_for_context::entity_error_kind::entity_error_kind::EntityErrorKind;
use crate::error::main_error_kind::main_error_kind::MainErrorKind;
use crate::repository::_in_context_for::entity::entity::application_user::application_user::_new_for_context::postgresql::base_repository::BaseRepository as ApplicationUserBaseRepository;
use crate::repository::_in_context_for::entity::entity::application_user::pre_confirmed_application_user::_new_for_context::postgresql::base_repository::BaseRepository as PreConfirmedApplicationUserBaseRepository;
use crate::repository::_in_context_for::entity::entity::application_user_registration_confirmation_token::_new_for_context::postgresql::base_repository::BaseRepository as ApplicationUserRegistrationConfirmationTokenBaseRepository;
use crate::utility::_in_context_for::diesel_component::_new_for_context::postgresql::connection_manager::ConnectionManager;

pub struct Handler;

impl<'outer> Handler {
    pub fn handle(request: Request) -> Result<HandlerResult, MainErrorKind> {
        let mut connection_manager: ConnectionManager = ConnectionManager::new();
        connection_manager.establish_connection()?;

        if !ApplicationUserBaseRepository::is_exist_by_nickanme(&connection_manager, request.get_nickname())? {
            match PreConfirmedApplicationUserBaseRepository::get_by_email(&connection_manager, request.get_email())? {
                Some(pre_confirmed_application_user) => {
                    match ApplicationUserRegistrationConfirmationTokenBaseRepository::get_by_pre_confirmed_application_user(&connection_manager, &pre_confirmed_application_user)? {
                        Some(application_user_registration_confirmation_token) => {
                            if !application_user_registration_confirmation_token.is_expired() {
                                if request.get_token() == application_user_registration_confirmation_token.get_value().get_value() {
                                   let application_user: ApplicationUser<'_> = ApplicationUser::new_from_pre_confirmed_application_user(&pre_confirmed_application_user, Nickname::new(request.nickname), Password::new(request.password));

                                   connection_manager.begin_transaction()?;
                                   match ApplicationUserBaseRepository::create(&connection_manager, &application_user) {
                                        Ok(_) => {
                                            match ApplicationUserRegistrationConfirmationTokenBaseRepository::delete(&connection_manager, &application_user_registration_confirmation_token) {
                                                Ok(_) => {
                                                    match PreConfirmedApplicationUserBaseRepository::delete(&connection_manager, &pre_confirmed_application_user) {
                                                        Ok(_) => {
                                                            connection_manager.commit_transaction()?;

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
                                        },
                                        Err(diesel_error_kind) => {
                                            connection_manager.rollback_transaction()?;

                                            return Err(diesel_error_kind)?;
                                        }
                                    };
                                } else {
                                    return Err(EntityErrorKind::ApplicationUserRegistrationConfirmationTokenErrorKind(ApplicationUserRegistrationConfirmationTokenErrorKind::InvalidValue))?;
                                }
                            } else {
                                return Err(EntityErrorKind::ApplicationUserRegistrationConfirmationTokenErrorKind(ApplicationUserRegistrationConfirmationTokenErrorKind::AlreadyExpired))?;
                            }
                        },
                        None => {
                            return Err(EntityErrorKind::ApplicationUserRegistrationConfirmationTokenErrorKind(ApplicationUserRegistrationConfirmationTokenErrorKind::NotFound))?;
                        }
                    };
                },
                None => {
                    if ApplicationUserBaseRepository::is_exist_by_email(&connection_manager, request.get_email())? {
                        return Err(EntityErrorKind::PreConfirmedApplicationUserErrorKind(PreConfirmedApplicationUserErrorKind::AlreadyRegistered))?;
                    } else {
                        return Err(EntityErrorKind::PreConfirmedApplicationUserErrorKind(PreConfirmedApplicationUserErrorKind::NotFound))?;
                    }
                }
            };
        } else {
            return Err(EntityErrorKind::ApplicationUserErrorKind(ApplicationUserErrorKind::AlreadyExist))?;
        }
    }
}