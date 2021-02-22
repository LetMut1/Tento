// use crate::diesel_component::model::entity::entity::application_user::existing::Existing;
// use crate::diesel_component::model::entity::entity::application_user::new::New;
// use crate::dto::actix_web_component::request_handler::api::version1::mobile::entity::entity::application_user::authorization::log_in::request::Request;
// use crate::entity::core::uuid_v4::UuidV4;
// use crate::entity::entity::application_user::application_user::ApplicationUser;
// use crate::entity::entity::application_user::core::email::Email;
// use crate::entity::entity::json_web_token::json_access_web_token::json_access_web_token::JsonAccessWebToken;
// use crate::entity::entity::json_web_token::json_refresh_web_token::json_refresh_web_token::JsonRefreshWebToken;
// use crate::repository::entity::entity::application_user::base_repository::BaseRepository;
// use crate::service::entity::entity::json_web_token::json_access_web_token::serialization_form_resolver::SerializationFormResolver;
// use crate::utility::repository::entity::_common::pg_connection_manager::PGConnectionManager;
// use maybe_owned::MaybeOwned;

// pub struct Handler<'a, 'b: 'a> {
//     base_repository: BaseRepository<'b>,
//     request: &'b Request,
//     serialization_form_resolver: SerializationFormResolver<'a>
// }

// impl<'a, 'b: 'a> Handler<'a, 'b> {
//     pub fn new(pg_connection_manager: &'b PGConnectionManager, request: &'b Request) -> Self {
//         return Self {
//             base_repository: BaseRepository::new(pg_connection_manager),
//             request,
//             serialization_form_resolver: SerializationFormResolver::new()
//         };
//     }

//     pub fn handle(&'a self) -> String {        // TODO Всплывание ошибок, В РекуестХэндлере делать try. 
//         let existing: &Existing = self.base_repository.get_by_email(&Email::new(MaybeOwned::Borrowed(self.request.get_email())));
//         // TODO check hashed password 


//         let json_refresh_web_token: JsonRefreshWebToken = 
//             JsonRefreshWebToken::new_from_credentials(&UuidV4::new_from_uuid(existing.get_id()), self.request.get_device_id().clone());
//     }
// }