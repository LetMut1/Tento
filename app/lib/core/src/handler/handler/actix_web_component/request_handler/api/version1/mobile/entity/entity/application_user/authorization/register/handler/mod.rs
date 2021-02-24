use crate::diesel_component::model::entity::entity::application_user::new::New;
use crate::dto::actix_web_component::request_handler::api::version1::mobile::entity::entity::application_user::authorization::register::request::Request;
use crate::entity::entity::application_user::application_user::ApplicationUser;
use crate::repository::entity::entity::application_user::base_repository::BaseRepository;
use crate::utility::repository::_common::pg_connection_manager::PGConnectionManager;

pub struct Handler;

impl<'b> Handler {
    pub fn handle(request: &'b Request) -> () {
        let application_user: ApplicationUser<'b> = ApplicationUser::new_from_credentials(   // TODO validate ememail  - Проставить самую легкую проверку, 
            request.get_email(), request.get_nickname(), request.get_password()
        );   
        let mut pg_connection_manager: PGConnectionManager = PGConnectionManager::new();
        pg_connection_manager.establish_connection();
        let base_repository: BaseRepository<'_> = BaseRepository::new(&pg_connection_manager);     // TODO Всплывание ошибок, В РекуестХэндлере делать try. 
        base_repository.save(&New::new_from_entity(&application_user));
        pg_connection_manager.close_connection();
    }
}       // TODO ОСТАНОВИЛСЯ ЗДЕСЬ. ИДТИ ВНИЗ ПО ДЕРЕВУ ДИРЕКТОРИЙ