use crate::diesel_component::model::entity::entity::application_user::new::New;
use crate::dto::actix_web_component::request_handler::api::version1::mobile::entity::entity::application_user::authorization::register::request::Request;
use crate::entity::entity::application_user::application_user::ApplicationUser;
use crate::repository::entity::entity::application_user::base_repository::BaseRepository;
use crate::utility::repository::_common::pg_connection_manager::PGConnectionManager;

pub struct Handler<'b> {
    pg_connection_manager: PGConnectionManager,
    request: &'b Request
}

impl<'a, 'b: 'a> Handler<'b> {
    pub fn new(request: &'b Request) -> Self {
        return Self {
            pg_connection_manager: PGConnectionManager::new(),
            request
        };
    }

    pub fn handle(&'a mut self) -> () {
        let application_user: ApplicationUser<'b> = ApplicationUser::new_from_credentials(   // TODO validate ememail  - Проставить самую легкую проверку, 
            self.request.get_email(), self.request.get_nickname(), self.request.get_password()
        );   
        let mut base_repository: BaseRepository<'_> = BaseRepository::new(&mut self.pg_connection_manager);     // TODO Всплывание ошибок, В РекуестХэндлере делать try. 
        base_repository.establish_connection();
        base_repository.save(&New::new_from_entity(&application_user));
        base_repository.close_connection();
    }
}