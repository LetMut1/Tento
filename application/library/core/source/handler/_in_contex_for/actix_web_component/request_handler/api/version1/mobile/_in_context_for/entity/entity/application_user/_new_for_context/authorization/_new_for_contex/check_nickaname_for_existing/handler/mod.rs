use crate::dto::request_parameters::_in_context_for::actix_web_component::request_handler::api::version1::mobile::_in_context_for::entity::entity::application_user::_new_for_context::authorization::_new_for_context::check_nickname_for_existing::query::Query;
use crate::dto::response_parameters::_in_context_for::handler::_in_context_for::actix_web_component::request_handler::api::version1::mobile::_in_context_for::entity::entity::application_user::_new_for_context::authorization::_new_for_context::check_nickaname_for_existing::handler::_new_for_context::result::Result as HandlerResult;
use crate::entity::entity::application_user::core::nickname::Nickname;
use crate::error::main_error_kind::main_error_kind::MainErrorKind;
use crate::repository::_in_context_for::entity::entity::application_user::_new_for_context::postgresql::base_repository::BaseRepository as ApplicationUserBaseRepository;
use crate::utility::_in_context_for::diesel_component::_new_for_context::postgresql::connection_manager::ConnectionManager;

pub struct Handler;

impl Handler {
    pub fn handle(query: Query) -> Result<HandlerResult, MainErrorKind> {
        let mut connection_manager: ConnectionManager = ConnectionManager::new();
        connection_manager.establish_connection()?;

        let handler_result: HandlerResult = HandlerResult::new(
            ApplicationUserBaseRepository::is_exist_by_nickanme(&connection_manager, &Nickname::new(query.application_user_nickname))?
        );
        
        connection_manager.close_connection();

        return Ok(handler_result);
    }
}