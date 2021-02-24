use crate::diesel_component::model::entity::entity::json_web_token::json_refresh_web_token::new::New;
use crate::diesel_component::schema::public::json_refresh_web_token;
use crate::utility::repository::_common::pg_connection_manager::PGConnectionManager;
// use diesel::dsl; 
// use diesel::ExpressionMethods;       // TODO delete if not necessary
// use diesel::QueryDsl;
use diesel::RunQueryDsl;

pub struct BaseRepository<'b> {              // TODO поменять внутренности под работу с Редисом при переходе на Редис. Сигнатуру методов, разумеется, не трогать (только если придется поменять работу с моделями)
    pg_connection_manager: &'b PGConnectionManager
}

impl<'a, 'b: 'a> BaseRepository<'b> {
    pub fn new(pg_connection_manager: &'b PGConnectionManager) -> Self {
        return Self {
            pg_connection_manager
        };
    }

    pub fn save(&'a self, new: &'b New) -> () {         // TODO ошибка alreadyExisting как перехватывать
        diesel::insert_into(json_refresh_web_token::table).values(new).execute(self.pg_connection_manager.get_connection()).unwrap();  //TODO ошибки, Плюс тру фолс, сохранилось ли или нет
    }
}