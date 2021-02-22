use crate::dto::actix_web_component::request_handler::api::version1::mobile::entity::entity::application_user::authorization::check_nickname_for_existing::query::Query;
use crate::entity::entity::application_user::core::nickname::Nickname;
use crate::repository::entity::entity::application_user::base_repository::BaseRepository;
use crate::utility::repository::entity::_common::pg_connection_manager::PGConnectionManager;
use maybe_owned::MaybeOwned;

pub struct Handler<'b> {
    base_repository: BaseRepository<'b>,
    query: &'b Query
}

impl<'a, 'b: 'a> Handler<'b> {
    pub fn new(pg_connection_manager: &'b PGConnectionManager, query: &'b Query) -> Self {
        return Self {
            base_repository: BaseRepository::new(pg_connection_manager),
            query
        };
    }

    pub fn handle(&'a self) -> bool {  // TODO Всплывание ошибок, В РекуестХэндлере делать try. 
        return self.base_repository.is_exist_by_nickanme(&Nickname::new(MaybeOwned::Borrowed(self.query.get_nickname())));
    }
}