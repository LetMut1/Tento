use crate::data_transfer_object::request_parameters::_in_context_for::actix_web_component::request_handler::api::version1::mobile::_in_context_for::entity::entity::application_user::_new_for_context::authorization::_new_for_context::refresh_json_access_web_token::request::Request;
use crate::data_transfer_object::response_parameters::_in_context_for::handler::_in_context_for::actix_web_component::request_handler::api::version1::mobile::_in_context_for::entity::entity::application_user::_new_for_context::authorization::_new_for_context::refresh_json_access_web_token::handler::_new_for_context::result::Result as HandlerResult;
use crate::entity::entity::json_access_web_token::json_access_web_token::JsonAccessWebToken;
use crate::error::main_error_kind::core::_in_context_for::entity::_new_for_context::entity_error_kind::core::_in_context_for::entity::json_access_web_token::_new_for_context::json_access_web_token_error_kind::JsonAccessWebTokenErrorKind;
use crate::error::main_error_kind::core::_in_context_for::entity::_new_for_context::entity_error_kind::core::_in_context_for::entity::json_refresh_web_token::_new_for_context::json_refresh_web_token_error_kind::JsonRefreshWebTokenErrorKind;
use crate::error::main_error_kind::core::_in_context_for::entity::_new_for_context::entity_error_kind::entity_error_kind::EntityErrorKind;
use crate::error::main_error_kind::core::invalid_argument_error::InvalidArgumentError;
use crate::error::main_error_kind::main_error_kind::MainErrorKind;
use crate::repository::_in_context_for::entity::entity::json_refresh_web_token::_new_for_context::postgresql::base_repository::BaseRepository as JsonRefreshWebTokenBaseRepository;
use crate::service::_in_context_for::entity::entity::json_access_web_token::_new_for_context::serialization_form_resolver::SerializationFormResolver;
use crate::service::_in_context_for::entity::entity::json_refresh_web_token::_new_for_context::encoder::Encoder;
use crate::utility::resource_connection::postgresql::connection_manager::ConnectionManager;

pub struct Handler;

impl Handler {
    pub fn handle(request: Request) -> Result<HandlerResult, MainErrorKind> {
        let mut json_access_web_token: JsonAccessWebToken<'_> = SerializationFormResolver::deserialize(request.json_access_web_token.as_str())?;

        if json_access_web_token.is_expired() {
            let mut connection_manager: ConnectionManager = ConnectionManager::new();
            connection_manager.establish_connection()?;

            if let Some (mut json_refresh_web_token) = JsonRefreshWebTokenBaseRepository::get_by_application_user_id_and_application_user_log_in_token_device_id(
                &connection_manager, json_access_web_token.get_application_user_id(), json_access_web_token.get_application_user_log_in_token_device_id()
            )?
            {
                if &(json_access_web_token.get_id().get_value().as_bytes())[..] == &(json_refresh_web_token.get_json_access_web_token_id().get_value().as_bytes())[..] {
                    if Encoder::is_valid(&json_refresh_web_token, request.json_refresh_web_token.as_str()) {
                        json_refresh_web_token.refresh();
                        json_access_web_token.refresh();

                        JsonRefreshWebTokenBaseRepository::update(&connection_manager, &json_refresh_web_token)?;

                        connection_manager.close_connection();

                        return Ok(
                            HandlerResult::new(
                                SerializationFormResolver::serialize(&json_access_web_token),
                                Encoder::encode(&json_refresh_web_token)
                            )
                        );
                    }
                }

                return Err(InvalidArgumentError)?;
            }

            return Err(EntityErrorKind::JsonRefreshWebTokenErrorKind(JsonRefreshWebTokenErrorKind::NotFound))?;
        }
        
        return Err(EntityErrorKind::JsonAccessWebTokenErrorKind(JsonAccessWebTokenErrorKind::NotExpired))?;
    }
}