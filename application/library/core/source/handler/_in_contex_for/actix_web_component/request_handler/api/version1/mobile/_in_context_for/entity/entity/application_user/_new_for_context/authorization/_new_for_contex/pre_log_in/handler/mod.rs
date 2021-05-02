use crate::data_transfer_object::request_parameters::_in_context_for::actix_web_component::request_handler::api::version1::mobile::_in_context_for::entity::entity::application_user::_new_for_context::authorization::_new_for_context::pre_log_in::request::Request;
use crate::data_transfer_object::response_parameters::_in_context_for::handler::_in_context_for::actix_web_component::request_handler::api::version1::mobile::_in_context_for::entity::entity::application_user::_new_for_context::authorization::_new_for_context::pre_log_in::handler::_new_for_context::result::Result as HandlerResult;
use crate::entity::core::uuid_v4::UuidV4;
use crate::entity::entity::application_user_log_in_token::application_user_log_in_token::ApplicationUserLogInToken;
use crate::entity::entity::application_user::core::email::Email;
use crate::entity::entity::application_user::core::password::Password;
use crate::error::main_error_kind::core::entity_error_kind::core::_in_context_for::entity::entity::application_user::_new_for_context::application_user_error_kind::ApplicationUserErrorKind;
use crate::error::main_error_kind::core::entity_error_kind::entity_error_kind::EntityErrorKind;
use crate::error::main_error_kind::main_error_kind::MainErrorKind;
use crate::repository::_in_context_for::entity::entity::application_user_log_in_token::_new_for_context::redis::base_repository::BaseRepository as ApplicationUserLogInTokenBaseRepository;
use crate::repository::_in_context_for::entity::entity::application_user::_new_for_context::postgresql::base_repository::BaseRepository as ApplicationUserBaseRepository;
use crate::service::_in_context_for::entity::entity::application_user::_new_for_context::email_sender::EmailSender;
use crate::utility::_in_context_for::entity::entity::application_user::core::password::_new_for_context::password_encoder::PasswordEncoder;
use crate::utility::resource_connection::postgresql::connection_manager::ConnectionManager as PostgresqlConnectionManager;
use crate::utility::resource_connection::redis::connection_manager::ConnectionManager as RedisConnectionManager;

pub struct Handler;

impl Handler {
    pub fn handle(request: Request) -> Result<HandlerResult, MainErrorKind> {
        let application_user_log_in_token_device_id: UuidV4 = UuidV4::new_from_string(request.application_user_log_in_token_device_id)?;

        let mut postgresql_connection_manager: PostgresqlConnectionManager = PostgresqlConnectionManager::new();
        postgresql_connection_manager.establish_connection()?;

        if let Some(application_user) = ApplicationUserBaseRepository::get_by_email(&postgresql_connection_manager, &Email::new(request.application_user_email))? {
            if PasswordEncoder::is_valid(&Password::new(request.application_user_password), application_user.get_passord_hash()) {
                let application_user_log_in_token: ApplicationUserLogInToken<'_>;

                let mut redis_connection_manager: RedisConnectionManager = RedisConnectionManager::new();
                redis_connection_manager.establish_connection()?;

                match ApplicationUserLogInTokenBaseRepository::get_by_application_user_id_and_device_id(
                    &mut redis_connection_manager, application_user.get_id(), &application_user_log_in_token_device_id
                )? 
                {
                    Some(existing_application_user_log_in_token) => {
                        application_user_log_in_token = existing_application_user_log_in_token;

                        ApplicationUserLogInTokenBaseRepository::update_expiration_time(&mut redis_connection_manager, &application_user_log_in_token)?;
                    },
                    None => {
                        application_user_log_in_token = ApplicationUserLogInToken::new(&application_user, application_user_log_in_token_device_id);

                        ApplicationUserLogInTokenBaseRepository::create(&mut redis_connection_manager, &application_user_log_in_token)?;
                    }
                }

                redis_connection_manager.close_connection();
                
                postgresql_connection_manager.close_connection();

                EmailSender::send_application_user_log_in_token(&application_user_log_in_token)?;

                return Ok(HandlerResult::new(application_user.get_id().get_value().to_string()));
            }
            
            return Err(MainErrorKind::EntityErrorKind(EntityErrorKind::ApplicationUserErrorKind(ApplicationUserErrorKind::WrongPassword)));
        }

        return Err(MainErrorKind::EntityErrorKind(EntityErrorKind::ApplicationUserErrorKind(ApplicationUserErrorKind::NotFound)));
    }
}