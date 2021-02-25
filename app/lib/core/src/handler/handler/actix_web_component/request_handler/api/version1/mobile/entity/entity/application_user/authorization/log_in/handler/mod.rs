use crate::diesel_component::model::entity::entity::application_user::existing::Existing as ApplicationUserExisting;
use crate::diesel_component::model::entity::entity::json_web_token::json_refresh_web_token::new::New as JsonRefreshWebTokenNew;
use crate::dto::actix_web_component::request_handler::api::version1::mobile::entity::entity::application_user::authorization::log_in::request::Request;
use crate::entity::entity::application_user::application_user::ApplicationUser;
use crate::entity::entity::json_web_token::json_access_web_token::json_access_web_token::JsonAccessWebToken;
use crate::entity::entity::json_web_token::json_refresh_web_token::json_refresh_web_token::JsonRefreshWebToken;
use crate::handler::returned_type::handler::actix_web_component::request_handler::api::version1::mobile::entity::entity::application_user::authorization::log_in::returned_type::ReturnedType;
use crate::repository::entity::entity::application_user::base_repository::BaseRepository as ApplicationUserBaseRepository;
use crate::repository::entity::entity::json_web_token::json_refresh_web_token::base_repository::BaseRepository as JsonRefreshWebTokenBaseRepository;
use crate::service::entity::entity::json_web_token::json_access_web_token::serialization_form_resolver::SerializationFormResolver;
use crate::utility::entity::entity::application_user::password_encoder::PasswordEncoder;
use crate::utility::repository::_common::pg_connection_manager::PGConnectionManager;
use maybe_owned::MaybeOwned;

pub struct Handler;

impl<'b> Handler{
    pub fn handle(request: &'b Request) -> ReturnedType {        // TODO Всплывание ошибок, В РекуестХэндлере делать try. 
        let returned_type: ReturnedType;
        let mut pg_connection_manager:PGConnectionManager = PGConnectionManager::new();
        pg_connection_manager.establish_connection();
        let application_user_existing: ApplicationUserExisting = 
            ApplicationUserBaseRepository::get_by_email(pg_connection_manager.get_connection(), request.get_email());
        let application_user: ApplicationUser<'_> = ApplicationUser::new_from_model(&application_user_existing);
        if  PasswordEncoder::is_valid(request.get_password(), application_user.get_passord_hash().get_value()) {
            if application_user.is_confirmed() {
                let json_refresh_web_token: JsonRefreshWebToken<'_, '_> = 
                    JsonRefreshWebToken::new_from_credentials(MaybeOwned::Borrowed(application_user.get_id()), request.get_device_id());   
                let json_refresh_web_token_new: JsonRefreshWebTokenNew<'_> = JsonRefreshWebTokenNew::new_from_entity(&json_refresh_web_token);
                JsonRefreshWebTokenBaseRepository::save(pg_connection_manager.get_connection(), &json_refresh_web_token_new);
                pg_connection_manager.close_connection(); 
                let json_access_web_token: JsonAccessWebToken<'_> = JsonAccessWebToken::new_from_json_refresh_web_token(&json_refresh_web_token);
                returned_type = ReturnedType::JsonAccessWebToken(SerializationFormResolver::serialize(&json_access_web_token));

                return returned_type;
            } else {
                returned_type = ReturnedType::NotConfirmed;
            }
        } else {
            returned_type = ReturnedType::WrongPassword;
        }
        pg_connection_manager.close_connection();

        return returned_type;
    }
}