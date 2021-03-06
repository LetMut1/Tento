use crate::diesel_component::model::entity::entity::application_user::new::New;
use crate::dto::actix_web_component::request_handler::api::version1::mobile::entity::entity::application_user::authorization::register::request::Request;
use crate::dto::handler::actix_web_component::request_handler::api::version1::mobile::entity::entity::application_user::authorization::register::handler::result::Result as HandlerResult;
use crate::entity::entity::application_user::application_user::ApplicationUser;
use crate::error::main_error_kind::core::entity::entity_error_kind::core::entity::application_user::application_user_error_kind::ApplicationUserErrorKind;
use crate::error::main_error_kind::core::entity::entity_error_kind::entity_error_kind::EntityErrorKind;
use crate::error::main_error_kind::main_error_kind::MainErrorKind;
use crate::repository::entity::entity::application_user::base_repository::BaseRepository;
use crate::utility::entity::entity::application_user::core::email::email_simple_validator::EmailSimpleValidator;
use crate::utility::repository::_common::pg_connection_manager::PGConnectionManager;

pub struct Handler;

impl<'outer> Handler {
    pub fn handle(request: &'outer Request) -> Result<HandlerResult, MainErrorKind> {
        if EmailSimpleValidator::is_valid(request.get_email()) {
            let application_user: ApplicationUser<'_> = ApplicationUser::new(
                request.get_email(), request.get_nickname(), request.get_password()
            );   
            let mut pg_connection_manager: PGConnectionManager = PGConnectionManager::new();
            pg_connection_manager.establish_connection()?;
            if !BaseRepository::is_exist_by_nickanme(pg_connection_manager.get_connection(), request.get_nickname())? 
                && !BaseRepository::is_exist_by_email(pg_connection_manager.get_connection(), request.get_email())?
            {
                BaseRepository::save(pg_connection_manager.get_connection(), &New::new_from_entity(&application_user))?;
                pg_connection_manager.close_connection();
            } else {
                pg_connection_manager.close_connection();

                return Err(EntityErrorKind::ApplicationUserErrorKind(ApplicationUserErrorKind::AlreadyExist))?;
            }
            
            return Ok(HandlerResult::new(true));
        } else {
            return Err(EntityErrorKind::ApplicationUserErrorKind(ApplicationUserErrorKind::InvalidEmail))?;
        }
    }
}