use crate::diesel_component::model::entity::entity::application_user::existing::Existing;
use crate::diesel_component::model::entity::entity::application_user::new::New;
use crate::diesel_component::schema::public::application_user;
use crate::utility::repository::_common::pg_connection_manager::PGConnectionManager;
use diesel::dsl; 
use diesel::ExpressionMethods;
use diesel::QueryDsl;
use diesel::RunQueryDsl;

pub struct BaseRepository<'b> {
    pg_connection_manager: &'b mut PGConnectionManager
}

impl<'a, 'b: 'a> BaseRepository<'b> {
    pub fn new(pg_connection_manager: &'b mut PGConnectionManager) -> Self {
        return Self {
            pg_connection_manager
        };
    }

    pub fn save(&'a self, new: &'b New) -> () {         // TODO ошибка alreadyExisting как перехватывать
        diesel::insert_into(application_user::table).values(new).execute(self.pg_connection_manager.get_connection()).unwrap();  //TODO ошибки, Плюс тру фолс, сохранилось ли или нет
    }

    pub fn is_exist_by_nickanme(&'a self, nickname: &'b String) -> bool { // TODO сделать возможномть устанавливать фильтр ?
        return 
            diesel::select(dsl::exists(application_user::table.filter(application_user::nickname.eq(nickname)))) // TODO посмотреть, что за запрос !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
                .get_result::<bool>(self.pg_connection_manager.get_connection()).unwrap();     // TODO ошибки
    }

    pub fn get_by_email(&'a mut self, email: &'b String) -> Existing {
        return 
            application_user::table.filter(application_user::email.eq(email))
                .limit(1).load::<Existing>(self.pg_connection_manager.get_connection()).unwrap()
                .pop().unwrap();
    }

    pub fn establish_connection(&'a mut self) -> () {
        self.pg_connection_manager.establish_connection();
    }

    pub fn close_connection(&'a mut self) -> () {
        self.pg_connection_manager.close_connection();
    }
}