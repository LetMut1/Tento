use crate::dto::actix_web_component::request_handler::api::version1::mobile::entity::entity::application_user::authorization::check_nickname_for_existing::query::Query;
use crate::repository::entity::entity::application_user::base_repository::BaseRepository;
use crate::utility::repository::entity::_common::pg_connection_manager::PGConnectionManager;

pub struct Handler<'b> {
    pg_connection_manager: PGConnectionManager,
    query: &'b Query
}

impl<'a, 'b: 'a> Handler<'b> {
    pub fn new(query: &'b Query) -> Self {
        return Self {
            pg_connection_manager: PGConnectionManager::new(),
            query
        };
    }

    pub fn handle(&'a self) -> bool {  // TODO Всплывание ошибок, В РекуестХэндлере делать try. 
        let base_repository: BaseRepository<'_> = BaseRepository::new(&self.pg_connection_manager);

        return base_repository.is_exist_by_nickanme(self.query.get_nickname());
    }
}