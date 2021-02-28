use crate::diesel_component::model::entity::entity::application_user::new::New;
use crate::dto::actix_web_component::request_handler::api::version1::mobile::entity::entity::application_user::authorization::register::request::Request;
use crate::entity::entity::application_user::application_user::ApplicationUser;
use crate::error::error::core::entity::entity_error_kind::EntityErrorKind;
use crate::error::error::core::entity::entity::application_user::application_user_error_kind::ApplicationUserErrorKind;
use crate::error::error::main_error_kind::MainErrorKind;
use crate::repository::entity::entity::application_user::base_repository::BaseRepository;
use crate::utility::repository::_common::pg_connection_manager::PGConnectionManager;

pub struct Handler;

impl<'outer> Handler {
    pub fn handle(request: &'outer Request) -> Result<(), MainErrorKind> {
        let application_user: ApplicationUser<'_> = ApplicationUser::new_from_credentials(   // TODO validate ememail  - Проставить самую легкую проверку, 
            request.get_email(), request.get_nickname(), request.get_password()
        );   
        let mut pg_connection_manager: PGConnectionManager = PGConnectionManager::new();
        pg_connection_manager.establish_connection()?;
        if BaseRepository::is_exist_by_nickanme(pg_connection_manager.get_connection(), request.get_nickname())? {  // TODO проверять по емейлу
            BaseRepository::save(pg_connection_manager.get_connection(), &New::new_from_entity(&application_user))?;
            pg_connection_manager.close_connection();
        } else {
            return Err(EntityErrorKind::ApplicationUserErrorKind(ApplicationUserErrorKind::AlreadyExist(
                Some(format!("For Nickname '{}'", request.get_nickname()))))
            )?
        }
        
        return Ok(());
    }
}