use crate::diesel_component::model::entity::entity::application_user::existing::Existing;
use crate::diesel_component::model::entity::entity::application_user::new::New;
use crate::diesel_component::schema::public::application_user;
use crate::entity::entity::application_user::core::email::Email;
use crate::entity::entity::application_user::core::nickname::Nickname;
use crate::utility::repository::entity::_common::pg_connection_manager::PGConnectionManager;
use diesel::dsl; 
use diesel::ExpressionMethods;
use diesel::QueryDsl;
use diesel::RunQueryDsl;

pub struct BaseRepository<'b> {
    pg_connection_manager: &'b PGConnectionManager,
    existing_registry: Option<Vec<Existing>>
}

impl<'a, 'b: 'a> BaseRepository<'b> {
    pub fn new(pg_connection_manager: &'b PGConnectionManager) -> Self {            // TODO разделить коре и mod.rs в сущностях
        return Self {
            pg_connection_manager,
            existing_registry: None
        };
    }

    pub fn save(&'a self, new: New) -> () {  // TODO всплывание ошибок
        diesel::insert_into(application_user::table).values(new).execute(self.pg_connection_manager.get_connection()).unwrap();  //TODO ошибки, Плюс тру фолс, сохранилось ли или нет
    }

    pub fn is_exist_by_nickanme(&'a self, nickname: &'b Nickname) -> bool { // TODO сделать возможномть устанавливать фильтр ?
        return 
            diesel::select( // TODO посмотреть, что за запрос !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
                dsl::exists(application_user::table.filter(application_user::nickname.eq(nickname.get_value())))
            ).get_result::<bool>(self.pg_connection_manager.get_connection()).unwrap();     // TODO ошибки
    }

    pub fn get_by_email(&'a mut self, email: &'b Email) -> &'a Existing {
        self.existing_registry = Some(application_user::table.filter(
            application_user::email.eq(email.get_value())
        ).limit(1).load::<Existing>(self.pg_connection_manager.get_connection()).unwrap()); // TODO ошибки
        match self.existing_registry {
            Some(ref existing_registry) => { return &(existing_registry[0]); },
            None => panic!("Logic error, please, initialize 'self.existing_registry' field with not-None value")
        };
    }

    pub fn get_pg_connection_manager(&'a self) -> &'b PGConnectionManager {
        return self.pg_connection_manager;
    }

    pub fn get_existing(&'a self) -> &'a Vec<Existing> {
        match self.existing_registry {
            Some(ref existing_registry) => { return existing_registry; },
            None => panic!("Logic error, please, initialize 'self.existing_registry' field with not-None value")
        };
    }
}