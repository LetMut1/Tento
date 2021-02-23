use crate::diesel_component::model::entity::entity::application_user::existing::Existing;
use crate::dto::actix_web_component::request_handler::api::version1::mobile::entity::entity::application_user::authorization::log_in::request::Request;
use crate::entity::entity::application_user::application_user::ApplicationUser;
use crate::entity::entity::json_web_token::json_access_web_token::json_access_web_token::JsonAccessWebToken;
use crate::entity::entity::json_web_token::json_refresh_web_token::json_refresh_web_token::JsonRefreshWebToken;
use crate::handler::returned_type::handler::actix_web_component::request_handler::api::version1::mobile::entity::entity::application_user::authorization::log_in::returned_type::ReturnedType;
use crate::repository::entity::entity::application_user::base_repository::BaseRepository;
use crate::service::entity::entity::json_web_token::json_access_web_token::serialization_form_resolver::SerializationFormResolver;
use crate::utility::entity::entity::application_user::password_encoder::PasswordEncoder;
use crate::utility::repository::_common::pg_connection_manager::PGConnectionManager;
use maybe_owned::MaybeOwned;

pub struct Handler<'a, 'b: 'a> {
    pg_connection_manager: PGConnectionManager,
    request: &'b Request,
    serialization_form_resolver: SerializationFormResolver<'a>,
    password_encoder: PasswordEncoder
}

impl<'a, 'b: 'a> Handler<'a, 'b> {
    pub fn new(request: &'b Request) -> Self {
        return Self {
            pg_connection_manager: PGConnectionManager::new(),
            request,
            serialization_form_resolver: SerializationFormResolver::new(),
            password_encoder: PasswordEncoder::new()
        };
    }

    pub fn handle(&'a mut self) -> ReturnedType {        // TODO Всплывание ошибок, В РекуестХэндлере делать try. 
        let returned_type: ReturnedType;
        let mut base_repository: BaseRepository<'_> = BaseRepository::new(&mut self.pg_connection_manager);
        base_repository.establish_connection();
        let existing: Existing = base_repository.get_by_email(self.request.get_email());
        let application_user: ApplicationUser<'_> = ApplicationUser::new_from_model(&existing);
        if self.password_encoder.is_valid(self.request.get_password(), application_user.get_passord_hash().get_value()) {
            if application_user.is_confirmed() {
                let json_refresh_web_token: JsonRefreshWebToken<'_, '_> = 
                    JsonRefreshWebToken::new_from_credentials(MaybeOwned::Borrowed(application_user.get_id()), self.request.get_device_id());   
                    // TODO cохраняем в бд
                base_repository.close_connection(); 
                let json_access_web_token: JsonAccessWebToken<'_> = JsonAccessWebToken::new_from_jrwt(&json_refresh_web_token);
                returned_type = ReturnedType::JsonAccessWebToken(self.serialization_form_resolver.serialize(&json_access_web_token));

                return returned_type;
            } else {
                returned_type = ReturnedType::NotConfirmed;
            }
        } else {
            returned_type = ReturnedType::WrongPassword;
        }
        base_repository.close_connection();

        return returned_type;
    }
}