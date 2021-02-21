// use crate::diesel_component::model::entity::entity::application_user::new::New;
// use crate::dto::actix_web_component::request_handler::api::version1::mobile::entity::entity::application_user::authorization::register::request::Request;
// use crate::entity::entity::application_user::ApplicationUser;
// use crate::repository::entity::entity::application_user::base_repository::BaseRepository;
// use crate::utility::repository::entity::common::pg_connection_manager::PGConnectionManager;

// pub struct Handler<'b> {
//     base_repository: BaseRepository<'b>,
//     request: &'b Request
// }

// impl<'a, 'b: 'a> Handler<'b> {
//     pub fn new(pg_connection_manager: &'b PGConnectionManager, request: &'b Request) -> Self {
//         return Self {
//             base_repository: BaseRepository::new(pg_connection_manager),
//             request
//         };
//     }

//     pub fn handle(&'a self) -> bool {                                                                  // TODO Всплывание ошибок, В РекуестХэндлере делать try. 
        
//     }
// }