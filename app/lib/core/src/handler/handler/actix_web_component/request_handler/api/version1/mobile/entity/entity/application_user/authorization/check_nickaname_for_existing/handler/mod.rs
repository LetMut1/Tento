use crate::dto::actix_web_component::request_handler::api::version1::mobile::entity::entity::application_user::authorization::check_nickname_for_existing::query::Query;
use crate::repository::entity::entity::application_user::base_repository::BaseRepository;
use crate::utility::repository::_common::pg_connection_manager::PGConnectionManager;

pub struct Handler;

impl<'outer> Handler {
    pub fn handle(query: &'outer Query) -> bool {  // TODO Всплывание ошибок, В РекуестХэндлере делать try.
        let mut pg_connection_manager: PGConnectionManager = PGConnectionManager::new();
        pg_connection_manager.establish_connection();
        let result: bool = BaseRepository::is_exist_by_nickanme(pg_connection_manager.get_connection(), query.get_nickname());
        pg_connection_manager.close_connection();

        return result;
    }
}