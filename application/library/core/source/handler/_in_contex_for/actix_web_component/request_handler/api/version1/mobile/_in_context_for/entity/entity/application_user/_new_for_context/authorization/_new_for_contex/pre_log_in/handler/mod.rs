use crate::data_transfer_object::request_parameters::_in_context_for::actix_web_component::request_handler::api::version1::mobile::_in_context_for::entity::entity::application_user::_new_for_context::authorization::_new_for_context::pre_log_in::request::Request;
use crate::data_transfer_object::response_parameters::_in_context_for::handler::_in_context_for::actix_web_component::request_handler::api::version1::mobile::_in_context_for::entity::entity::application_user::_new_for_context::authorization::_new_for_context::pre_log_in::handler::_new_for_context::result::Result as HandlerResult;
use crate::entity::entity::application_user_log_in_token::application_user_log_in_token::ApplicationUserLogInToken;
use crate::entity::entity::application_user_log_in_token::core::device_id::DeviceId as ApplicationUserLogInTokenDeviceId;
use crate::entity::entity::application_user::core::email::Email;
use crate::entity::entity::application_user::core::password::Password;
use crate::error::main_error_kind::core::entity_error_kind::core::_in_context_for::entity::entity::application_user::_new_for_context::application_user_error_kind::ApplicationUserErrorKind;
use crate::error::main_error_kind::core::entity_error_kind::entity_error_kind::EntityErrorKind;
use crate::error::main_error_kind::main_error_kind::MainErrorKind;
use crate::repository::_in_context_for::entity::entity::application_user_log_in_token::_new_for_context::_in_context_for::_resource::redis::_new_for_context::base_repository::BaseRepository as ApplicationUserLogInTokenBaseRepository;
use crate::repository::_in_context_for::entity::entity::application_user::_new_for_context::_in_context_for::_resource::postgresql::_new_for_context::base_repository::BaseRepository as ApplicationUserBaseRepository;
use crate::service::_in_context_for::entity::entity::application_user::_new_for_context::email_sender::EmailSender;
use crate::utility::_in_context_for::_resource::_new_for_context::aggregate_connection_pool::AggregateConnectionPool;
use crate::utility::_in_context_for::_resource::_new_for_context::connection_extractor::ConnectionExtractor;
use crate::utility::_in_context_for::entity::entity::application_user::core::password::_new_for_context::password_encoder::PasswordEncoder;
use redis::Connection;
use std::sync::Arc;

pub struct Handler;

impl Handler {
    pub fn handle(request: Request, aggregate_connection_pool: Arc<AggregateConnectionPool>) -> Result<HandlerResult, MainErrorKind> {
        let application_user_log_in_token_device_id: ApplicationUserLogInTokenDeviceId =
        ApplicationUserLogInTokenDeviceId::new_from_string(request.application_user_log_in_token_device_id)?;

        if let Some(application_user) = ApplicationUserBaseRepository::get_by_email(
            &*ConnectionExtractor::get_postgresql_connection(&aggregate_connection_pool)?, &Email::new(request.application_user_email)
        )? 
        {
            if PasswordEncoder::is_valid(&Password::new(request.application_user_password), application_user.get_passord_hash()) {
                let application_user_log_in_token: ApplicationUserLogInToken<'_>;

                let connection: &'_ mut Connection = &mut *ConnectionExtractor::get_redis_connection(&aggregate_connection_pool)?;

                match ApplicationUserLogInTokenBaseRepository::get_by_application_user_id_and_device_id(
                    connection, application_user.get_id(), &application_user_log_in_token_device_id
                )? 
                {
                    Some(existing_application_user_log_in_token) => {
                        application_user_log_in_token = existing_application_user_log_in_token;

                        ApplicationUserLogInTokenBaseRepository::update_expiration_time(connection, &application_user_log_in_token)?;
                    },
                    None => {
                        application_user_log_in_token = ApplicationUserLogInToken::new(&application_user, &application_user_log_in_token_device_id);

                        ApplicationUserLogInTokenBaseRepository::create(connection, &application_user_log_in_token)?;
                    }
                }
                

                EmailSender::send_application_user_log_in_token(&application_user_log_in_token)?;

                return Ok(HandlerResult::new(application_user.get_id().to_string()));
            }
            
            return Err(MainErrorKind::EntityErrorKind(EntityErrorKind::ApplicationUserErrorKind(ApplicationUserErrorKind::WrongPassword)));
        }

        return Err(MainErrorKind::EntityErrorKind(EntityErrorKind::ApplicationUserErrorKind(ApplicationUserErrorKind::NotFound)));
    }
}