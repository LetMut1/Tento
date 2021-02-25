use crate::diesel_component::model::entity::entity::json_web_token::json_refresh_web_token::new::New;
use crate::diesel_component::schema::public::json_refresh_web_token;
use diesel::pg::PgConnection;
use diesel::RunQueryDsl;

pub struct BaseRepository;

impl<'outer> BaseRepository {
    pub fn save(pg_connection_manager: &'outer PgConnection, new: &'outer New) -> () {      // TODO ошибка alreadyExisting как перехватывать
        diesel::insert_into(json_refresh_web_token::table).values(new).execute(pg_connection_manager).unwrap();  //TODO ошибки, Плюс тру фолс, сохранилось ли или нет
    }
}