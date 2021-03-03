use crate::dto::actix_web_component::request_handler::api::version1::mobile::entity::entity::application_user::authorization::check_email_for_existing::query::Query;
use crate::error::main_error_kind::core::diesel_component::diesel_error_kind::DieselErrorKind;
use crate::error::main_error_kind::main_error_kind::MainErrorKind;
use crate::repository::entity::entity::application_user::base_repository::BaseRepository;
use crate::utility::repository::_common::pg_connection_manager::PGConnectionManager;

pub struct Handler;

impl<'outer> Handler {
    pub fn handle(query: &'outer Query) -> Result<bool, MainErrorKind> {
        let mut pg_connection_manager: PGConnectionManager = PGConnectionManager::new();
        pg_connection_manager.establish_connection()?;
        let result: Result<bool, DieselErrorKind> = 
            BaseRepository::is_exist_by_email(pg_connection_manager.get_connection(), query.get_email());
        pg_connection_manager.close_connection();

        return Ok(result?);
    }
}