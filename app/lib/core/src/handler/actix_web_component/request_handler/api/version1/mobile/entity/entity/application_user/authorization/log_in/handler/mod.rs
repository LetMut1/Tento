// use crate::diesel_component::model::entity::entity::application_user::new::New;
// use crate::diesel_component::model::entity::entity::application_user::existing::Existing;
// use crate::dto::actix_web_component::request_handler::api::version1::mobile::entity::entity::application_user::authorization::log_in::request::Request;
// use crate::entity::entity::application_user::ApplicationUser;
// use crate::repository::entity::entity::application_user::base_repository::BaseRepository;
// use crate::utility::repository::entity::_common::pg_connection_manager::PGConnectionManager;
// use crate::service::entity::entity::json_web_token::json_access_web_token::serialization_form_resolver::SerializationFormResolver;
// use crate::entity::entity::json_web_token::json_access_web_token::JsonAccessWebToken;

// pub struct Handler<'b> {
//     base_repository: BaseRepository<'b>,
//     request: &'b Request,
// }

// impl<'a, 'b: 'a> Handler<'b> {
//     pub fn new(pg_connection_manager: &'b PGConnectionManager, request: &'b Request) -> Self {
//         return Self {
//             base_repository: BaseRepository::new(pg_connection_manager),
//             request
//         };
//     }

//     pub fn handle(&'a self) -> String {                                                                  // TODO Всплывание ошибок, В РекуестХэндлере делать try. 
        
//     }
// }