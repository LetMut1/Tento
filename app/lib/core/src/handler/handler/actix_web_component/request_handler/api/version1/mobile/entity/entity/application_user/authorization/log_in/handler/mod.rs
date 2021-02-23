use crate::diesel_component::model::entity::entity::application_user::existing::Existing;
use crate::dto::actix_web_component::request_handler::api::version1::mobile::entity::entity::application_user::authorization::log_in::request::Request;
use crate::entity::core::uuid_v4::UuidV4;
use crate::entity::entity::application_user::core::email::Email;
use crate::entity::entity::json_web_token::json_access_web_token::json_access_web_token::JsonAccessWebToken;
use crate::entity::entity::json_web_token::json_refresh_web_token::json_refresh_web_token::JsonRefreshWebToken;
use crate::handler::returned_type::handler::actix_web_component::request_handler::api::version1::mobile::entity::entity::application_user::authorization::log_in::returned_type::ReturnedType;
use crate::repository::entity::entity::application_user::base_repository::BaseRepository;
use crate::service::entity::entity::json_web_token::json_access_web_token::serialization_form_resolver::SerializationFormResolver;
use crate::utility::entity::entity::application_user::password_encoder::PasswordEncoder;
use crate::utility::repository::entity::_common::pg_connection_manager::PGConnectionManager;
use maybe_owned::MaybeOwned;

pub struct Handler<'a, 'b: 'a> {
    base_repository: BaseRepository<'b>,
    request: &'b Request,
    serialization_form_resolver: SerializationFormResolver<'a>,
    password_encoder: PasswordEncoder
}

impl<'a, 'b: 'a> Handler<'a, 'b> {
    pub fn new(pg_connection_manager: &'b PGConnectionManager, request: &'b Request) -> Self {      // TODO посмотреть везед по коду и убрать добавленное заимствование
        return Self {
            base_repository: BaseRepository::new(pg_connection_manager),
            request,
            serialization_form_resolver: SerializationFormResolver::new(),
            password_encoder: PasswordEncoder::new()
        };
    }

    pub fn handle(&'a mut self) -> ReturnedType {        // TODO Всплывание ошибок, В РекуестХэндлере делать try. 
        let existing: &Existing = self.base_repository.get_by_email(&Email::new(MaybeOwned::Borrowed(self.request.get_email())));
        if self.password_encoder.is_valid(self.request.get_password(), existing.get_password_hash()) { // TODO // TODO // TODO check CONFIRMED !!!!!!!!!!!!!!!!!!
            let json_refresh_web_token: JsonRefreshWebToken<'_, '_> = 
            JsonRefreshWebToken::new_from_credentials(UuidV4::new_from_uuid(existing.get_id()), self.request.get_device_id());   // TODO cохраняем в бд 
            let json_access_web_token: JsonAccessWebToken<'_> = JsonAccessWebToken::new_from_jrwt(&json_refresh_web_token);
            
            return ReturnedType::JsonAccessWebToken(self.serialization_form_resolver.serialize(&json_access_web_token));
        } else {
            return ReturnedType::NotConfirmed;
        }
    }
}               // TODO сохраняем в бд !!!!!!!!!!!!!!!!!!!!!!!!!!