use crate::data_transfer_object::request_parameters::_in_context_for::actix_web_component::request_handler::api::version1::mobile::_in_context_for::entity::entity::application_user::_new_for_context::authorization::_new_for_context::log_in::request::Request;
use crate::data_transfer_object::response_parameters::_in_context_for::handler::_in_context_for::actix_web_component::request_handler::api::version1::mobile::_in_context_for::entity::entity::application_user::_new_for_context::authorization::_new_for_context::log_in::handler::_new_for_context::result::Result as HandlerResult;
use crate::entity::core::uuid_v4::UuidV4;
use crate::entity::entity::json_access_web_token_black_list::json_access_web_token_black_list::JsonAccessWebTokenBlackList;
use crate::entity::entity::json_access_web_token::json_access_web_token::JsonAccessWebToken;
use crate::entity::entity::json_refresh_web_token::json_refresh_web_token::JsonRefreshWebToken;
use crate::error::main_error_kind::core::entity_error_kind::core::_in_context_for::entity::entity::application_user_log_in_token::_new_for_context::application_user_log_in_token::ApplicationUserLogInTokenErrorKind;
use crate::error::main_error_kind::core::entity_error_kind::entity_error_kind::EntityErrorKind;
use crate::error::main_error_kind::main_error_kind::MainErrorKind;
use crate::repository::_in_context_for::entity::entity::application_user_log_in_token::_new_for_context::postgresql::base_repository::BaseRepository as ApplicationUserLogInTokenBaseRepository;
use crate::repository::_in_context_for::entity::entity::json_access_web_token_black_list::_new_for_context::postgresql::base_repository::BaseRepository as JsonAccessWebTokenBlackListRepository;
use crate::repository::_in_context_for::entity::entity::json_refresh_web_token::_new_for_context::postgresql::base_repository::BaseRepository as JsonRefreshWebTokenBaseRepository;
use crate::service::_in_context_for::entity::entity::json_access_web_token::_new_for_context::serialization_form_resolver::SerializationFormResolver;
use crate::service::_in_context_for::entity::entity::json_refresh_web_token::_new_for_context::encoder::Encoder;
use crate::utility::resource_connection::postgresql::connection_manager::ConnectionManager as PostgresqlConnectionManager;
use crate::utility::resource_connection::redis::connection_manager::ConnectionManager as RedisConnectionManager;

pub struct Handler;

impl Handler {
    pub fn handle(request: Request) -> Result<HandlerResult, MainErrorKind> {   // TODO сделать На Редисе механизм для невозможности почстоянно отравки емэйла. (Сохранять, если отправлено, и проверять, что отпрпавили. удалять по времени)
        let application_user_id: UuidV4 = UuidV4::new_from_string(request.application_user_id)?;

        let application_user_log_in_token_device_id: UuidV4 = UuidV4::new_from_string(request.application_user_log_in_token_device_id)?;

        let mut redis_connection_manager: RedisConnectionManager = RedisConnectionManager::new();
        redis_connection_manager.establish_connection()?;

        if let Some(application_user_log_in_token) = ApplicationUserLogInTokenBaseRepository::get_by_application_user_id_and_device_id(
            &mut redis_connection_manager, &application_user_id, &application_user_log_in_token_device_id
        )?
        {
            let mut postgresql_connection_manager: PostgresqlConnectionManager = PostgresqlConnectionManager::new();
            postgresql_connection_manager.establish_connection()?;

            if application_user_log_in_token.get_value().get_value() == request.application_user_log_in_token_value.as_str() {
                if !application_user_log_in_token.is_expired() {
                    
                    if let Some(existing_json_refresh_web_token) = JsonRefreshWebTokenBaseRepository::get_by_application_user_id_and_application_user_log_in_token_device_id(
                        &mut redis_connection_manager, application_user_log_in_token.get_application_user_id(), application_user_log_in_token.get_device_id()
                    )? 
                    {
                        JsonAccessWebTokenBlackListRepository::create(
                            &mut redis_connection_manager, &JsonAccessWebTokenBlackList::new(existing_json_refresh_web_token.get_json_access_web_token_id())
                        )?;

                        JsonRefreshWebTokenBaseRepository::delete(&mut redis_connection_manager, &existing_json_refresh_web_token)?;
                    }

                    let json_refresh_web_token: JsonRefreshWebToken<'_> =
                    JsonRefreshWebToken::new(application_user_log_in_token.get_application_user_id(), application_user_log_in_token.get_device_id());

                    postgresql_connection_manager.begin_transaction()?;
                    
                    if let Err(resource_error_kind) = ApplicationUserLogInTokenBaseRepository::delete(&mut redis_connection_manager, &application_user_log_in_token) { 
                        postgresql_connection_manager.rollback_transaction()?;

                        return Err(MainErrorKind::ResourceErrorKind(resource_error_kind));
                        
                    }

                    if let Err(resource_error_kind) = JsonRefreshWebTokenBaseRepository::create(&mut redis_connection_manager, &json_refresh_web_token) {
                        postgresql_connection_manager.rollback_transaction()?;

                        return Err(MainErrorKind::ResourceErrorKind(resource_error_kind));
                    }

                    postgresql_connection_manager.commit_transaction()?;
                    postgresql_connection_manager.close_connection();

                    return Ok(
                        HandlerResult::new(
                            SerializationFormResolver::serialize(&JsonAccessWebToken::new(&json_refresh_web_token)),
                            Encoder::encode(&json_refresh_web_token)
                        )
                    );
                }
                
                return Err(MainErrorKind::EntityErrorKind(EntityErrorKind::ApplicationUserLogInTokenErrorKind(ApplicationUserLogInTokenErrorKind::AlreadyExpired)));
            }
            
            return Err(MainErrorKind::EntityErrorKind(EntityErrorKind::ApplicationUserLogInTokenErrorKind(ApplicationUserLogInTokenErrorKind::InvalidValue)));
        }

        return Err(MainErrorKind::EntityErrorKind(EntityErrorKind::ApplicationUserLogInTokenErrorKind(ApplicationUserLogInTokenErrorKind::NotFound)));
    }
}