use crate::dto::_in_context_for::actix_web_component::request_handler::api::version1::mobile::_in_context_for::entity::entity::application_user::_new_for_context::authorization::_new_for_context::check_nickname_for_existing::query::Query;
use crate::dto::_in_context_for::handler::_in_context_for::actix_web_component::request_handler::api::version1::mobile::_in_context_for::entity::entity::application_user::_new_for_context::authorization::_new_for_context::check_nickaname_for_existing::handler::_new_for_context::result::Result as HandlerResult;
use crate::error::main_error_kind::core::_in_context_for::diesel_component::_new_for_context::diesel_error_kind::DieselErrorKind;
use crate::error::main_error_kind::main_error_kind::MainErrorKind;
use crate::repository::_in_context_for::entity::entity::application_user::_new_for_context::base_repository::BaseRepository;
use crate::utility::_in_context_for::repository::_new_for_context::pg_connection_manager::PGConnectionManager;

pub struct Handler;

impl<'outer> Handler {
    pub fn handle(query: &'outer Query) -> Result<HandlerResult, MainErrorKind> {
        let mut pg_connection_manager: PGConnectionManager = PGConnectionManager::new();
        pg_connection_manager.establish_connection()?;
        let result: Result<bool, DieselErrorKind> = 
            BaseRepository::is_exist_by_nickanme(pg_connection_manager.get_connection(), query.get_nickname());
        pg_connection_manager.close_connection();

        return Ok(HandlerResult::new(result?));
    }
}