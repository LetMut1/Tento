use crate::diesel_component::model::entity::entity::application_user::existing::Existing;
use crate::diesel_component::model::entity::entity::application_user::new::New;
use crate::diesel_component::schema::public::application_user;
use diesel::dsl; 
use diesel::ExpressionMethods;
use diesel::pg::PgConnection;
use diesel::QueryDsl;
use diesel::RunQueryDsl;

pub struct BaseRepository;

impl<'outer> BaseRepository {
    pub fn save(pg_connection_manager: &'outer PgConnection, new: &'outer New) -> () {         // TODO ошибка alreadyExisting как перехватывать
        diesel::insert_into(application_user::table).values(new).execute(pg_connection_manager).unwrap();  //TODO ошибки, Плюс тру фолс, сохранилось ли или нет
    }

    pub fn is_exist_by_nickanme(pg_connection_manager: &'outer PgConnection, nickname: &'outer String) -> bool { // TODO сделать возможномть устанавливать фильтр ?
        return 
            diesel::select(dsl::exists(application_user::table.filter(application_user::nickname.eq(nickname)))) // TODO посмотреть, что за запрос !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
                .get_result::<bool>(pg_connection_manager).unwrap();     // TODO ошибки
    }

    pub fn get_by_email(pg_connection_manager: &'outer PgConnection, email: &'outer String) -> Existing {
        return 
            application_user::table.filter(application_user::email.eq(email))
                .limit(1).load::<Existing>(pg_connection_manager).unwrap()      // TODO ошибки
                .pop().unwrap();
    }
}