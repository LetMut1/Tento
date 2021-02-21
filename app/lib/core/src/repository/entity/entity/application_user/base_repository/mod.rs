use diesel::RunQueryDsl;

use crate::diesel_component::model::entity::entity::application_user::new::New;
use crate::diesel_component::schema::public::application_user::dsl::application_user;
use crate::utility::repository::entity::_common::pg_connection_manager::PGConnectionManager;

pub struct BaseRepository<'b> {
    pg_connection_manager: &'b PGConnectionManager
}

impl<'a, 'b: 'a> BaseRepository<'b> {
    pub fn new(pg_connection_manager: &'b PGConnectionManager) -> Self {
        return Self {
            pg_connection_manager
        };
    }

    pub fn save(&'a self, new: New) -> () {  // TODO всплывание ошибок
        diesel::insert_into(application_user).values(new).execute(self.pg_connection_manager.get_connection()).unwrap();  //TODO ошибки
    }

    fn get_pg_connection_manager(&'a self) -> &'b PGConnectionManager {
        return self.pg_connection_manager;
    }
}