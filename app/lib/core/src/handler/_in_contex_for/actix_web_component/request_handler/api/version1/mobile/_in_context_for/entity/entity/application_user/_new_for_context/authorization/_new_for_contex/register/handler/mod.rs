use crate::dto::_in_context_for::actix_web_component::request_handler::api::version1::mobile::_in_context_for::entity::entity::application_user::_new_for_context::authorization::_new_for_context::register::request::Request;
use crate::dto::_in_context_for::handler::_in_context_for::actix_web_component::request_handler::api::version1::mobile::_in_context_for::entity::entity::application_user::_new_for_context::authorization::_new_for_context::register::handler::_new_for_context::result::Result as HandlerResult;
use crate::entity::entity::application_user_registration_confirmation_token::application_user_registration_confirmation_token::ApplicationUserRegistrationConfirmationToken;
use crate::entity::entity::application_user::application_user::ApplicationUser;
use crate::error::main_error_kind::core::_in_context_for::diesel_component::_new_for_context::diesel_error_kind::DieselErrorKind;
use crate::error::main_error_kind::core::_in_context_for::entity::_new_for_context::entity_error_kind::core::_in_context_for::entity::application_user::_new_for_context::application_user_error_kind::ApplicationUserErrorKind;
use crate::error::main_error_kind::core::_in_context_for::entity::_new_for_context::entity_error_kind::entity_error_kind::EntityErrorKind;
use crate::error::main_error_kind::main_error_kind::MainErrorKind;
use crate::repository::_in_context_for::entity::entity::application_user::_new_for_context::base_repository::BaseRepository;
use crate::resourse_model::_in_context_for::entity::entity::application_user_registration_confirmation_token::_new_for_context::new::New as ApplicationUserRegistrationConfirmationTokenNew;
use crate::resourse_model::_in_context_for::entity::entity::application_user::_new_for_context::new::New as ApplicationUserNew;
use crate::utility::_in_context_for::diesel_component::_new_for_context::postgresql::connection_manager::ConnectionManager;
use crate::utility::_in_context_for::entity::entity::application_user::core::email::_new_for_context::email_simple_validator::EmailSimpleValidator;
use diesel::Connection;
use diesel::connection::AnsiTransactionManager;
use diesel::connection::TransactionManager;

pub struct Handler;

impl<'outer> Handler {
    pub fn handle(request: &'outer Request) -> Result<HandlerResult, MainErrorKind> {
        if EmailSimpleValidator::is_valid(request.get_email()) {
            let application_user: ApplicationUser<'_> = ApplicationUser::new(request.get_email(), request.get_nickname(), request.get_password());  

            let application_user_registration_confirmation_token: ApplicationUserRegistrationConfirmationToken<'_> = ApplicationUserRegistrationConfirmationToken::new(&application_user);

            let mut connection_manager: ConnectionManager = ConnectionManager::new();
            connection_manager.establish_connection()?;

            let ansi_transaction_manager: &'_ AnsiTransactionManager = connection_manager.get_connection().transaction_manager();
            match ansi_transaction_manager.begin_transaction(connection_manager.get_connection()) {
                Ok(_value) => {}    // TODO можно ли обработать только ошибку
                Err(value) => { return Err(DieselErrorKind::new_any(value, None))?; }
            };

            if !BaseRepository::is_exist_by_nickanme(connection_manager.get_connection(), request.get_nickname())? 
                && !BaseRepository::is_exist_by_email(connection_manager.get_connection(), request.get_email())?
            {
                BaseRepository::save(connection_manager.get_connection(), &ApplicationUserNew::new_from_entity(&application_user))?;
                
                connection_manager.close_connection();
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