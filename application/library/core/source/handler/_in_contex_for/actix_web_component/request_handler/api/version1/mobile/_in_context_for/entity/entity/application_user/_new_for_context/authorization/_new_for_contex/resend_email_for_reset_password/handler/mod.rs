use crate::data_transfer_object::request_parameters::_in_context_for::actix_web_component::request_handler::api::version1::mobile::_in_context_for::entity::entity::application_user::_new_for_context::authorization::_new_for_context::resend_email_for_reset_password::request::Request;
use crate::entity::entity::application_user::core::id::Id as ApplicationUserId;
use crate::error::main_error_kind::core::entity_error_kind::core::_in_context_for::entity::entity::application_user_reset_password_token::_new_for_context::application_user_reset_password_token_error_kind::ApplicationUserResetPasswordTokenErrorKind;
use crate::error::main_error_kind::core::entity_error_kind::entity_error_kind::EntityErrorKind;
use crate::error::main_error_kind::main_error_kind::MainErrorKind;
use crate::repository::_in_context_for::entity::entity::application_user_reset_password_token::_new_for_context::_in_context_for::_resource::redis::_new_for_context::base_repository::BaseRepository as ApplicationUserResetPasswordTokenBaseRepository;
use crate::service::_in_context_for::entity::entity::application_user::_new_for_context::email_sender::EmailSender;
use crate::utility::_in_context_for::_resource::_new_for_context::aggregate_connection_pool::AggregateConnectionPool;
use crate::utility::_in_context_for::_resource::_new_for_context::connection_extractor::ConnectionExtractor;
use redis::Connection;
use std::sync::Arc;

pub struct Handler;

impl<'outer_a> Handler {
    pub fn handle(request: Request, aggregate_connection_pool: Arc<AggregateConnectionPool>) -> Result<(), MainErrorKind> {
        let application_user_id: ApplicationUserId = ApplicationUserId::new_from_string(request.application_user_id)?;

        let connection: &'_ mut Connection = &mut *ConnectionExtractor::get_redis_connection(&aggregate_connection_pool)?;

        if let Some(application_user_reset_password_token) = ApplicationUserResetPasswordTokenBaseRepository::get_by_application_user_id(connection, &application_user_id)? {

            EmailSender::send_application_user_reset_password_token(&application_user_reset_password_token)?;
    
            return Ok(());
        }

        return Err(MainErrorKind::EntityErrorKind(EntityErrorKind::ApplicationUserResetPasswordTokenErrorKind(ApplicationUserResetPasswordTokenErrorKind::NotFound)));
    }
}