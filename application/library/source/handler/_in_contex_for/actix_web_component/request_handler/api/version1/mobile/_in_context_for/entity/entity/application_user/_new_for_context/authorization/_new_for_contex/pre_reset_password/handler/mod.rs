use crate::data_transfer_object::request_parameters::_in_context_for::actix_web_component::request_handler::api::version1::mobile::_in_context_for::entity::entity::application_user::_new_for_context::authorization::_new_for_context::pre_reset_password::request::Request;
use crate::data_transfer_object::response_parameters::_in_context_for::handler::_in_context_for::actix_web_component::request_handler::api::version1::mobile::_in_context_for::entity::entity::application_user::_new_for_context::authorization::_new_for_context::pre_reset_password::handler::_new_for_context::result::Result as HandlerResult;
use crate::entity::entity::application_user_reset_password_token::application_user_reset_password_token::ApplicationUserResetPasswordToken;
use crate::entity::entity::application_user::_core::email::Email;
use crate::error::main_error::_core::entity_error::_core::_in_context_for::entity::entity::application_user::_new_for_context::application_user_error::ApplicationUserError;
use crate::error::main_error::_core::entity_error::entity_error::EntityError;
use crate::error::main_error::main_error::MainError;
use crate::repository::_in_context_for::entity::entity::application_user_reset_password_token::_new_for_context::_in_context_for::_resource::redis::_new_for_context::base_repository::BaseRepository as ApplicationUserResetPasswordTokenBaseRepository;
use crate::repository::_in_context_for::entity::entity::application_user::_new_for_context::_in_context_for::_resource::postgresql::_new_for_context::base_repository::BaseRepository as ApplicationUserBaseRepository;
use crate::service::_in_context_for::entity::entity::application_user::_new_for_context::email_sender::EmailSender;
use crate::utility::_in_context_for::_resource::_new_for_context::aggregate_connection_pool::AggregateConnectionPool;
use crate::utility::_in_context_for::_resource::_new_for_context::connection_extractor::ConnectionExtractor;
use redis::Connection;
use std::sync::Arc;

pub struct Handler;

impl Handler {
    pub fn handle(aggregate_connection_pool: Arc<AggregateConnectionPool>, request: Request) -> Result<HandlerResult, MainError> {
        if let Some(application_user) = ApplicationUserBaseRepository::get_by_email(
            &*ConnectionExtractor::get_postgresql_connection(&aggregate_connection_pool)?, &Email::new(request.application_user_email)
        )? 
        {
            let application_user_reset_password_token: ApplicationUserResetPasswordToken<'_>;

            let redis_connection: &'_ mut Connection = &mut *ConnectionExtractor::get_redis_connection(&aggregate_connection_pool)?;

            match ApplicationUserResetPasswordTokenBaseRepository::get_by_application_user_id(redis_connection, application_user.get_id())? {
                Some(existing_application_user_reset_password_token) => {
                    application_user_reset_password_token = existing_application_user_reset_password_token;

                    ApplicationUserResetPasswordTokenBaseRepository::update_expiration_time(redis_connection, &application_user_reset_password_token)?;
                },
                None => {
                    application_user_reset_password_token = ApplicationUserResetPasswordToken::new(&application_user);

                    ApplicationUserResetPasswordTokenBaseRepository::create(redis_connection, &application_user_reset_password_token)?;
                }
            }

            EmailSender::send_application_user_reset_password_token(&application_user_reset_password_token)?;

            return Ok(HandlerResult::new(application_user.get_id().to_string()));
        }

        return Err(MainError::EntityError(EntityError::ApplicationUserError(ApplicationUserError::NotFound)));
    }
}