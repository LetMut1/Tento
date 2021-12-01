use crate::domain_layer::error::entity_error::_component::_in_context_for::domain_layer::entity::application_user_reset_password_token::_new_for_context::application_user_reset_password_token_error::ApplicationUserResetPasswordTokenError;
use crate::domain_layer::error::entity_error::entity_error::EntityError;
use crate::domain_layer::repository::data_provider::_in_context_for::domain_layer::entity::application_user_reset_password_token::_new_for_context::_in_context_for::_resource::redis::_new_for_context::base_trait::BaseTrait as ApplicationUserResetPasswordTokenDataProviderRedisTrait;
use crate::domain_layer::service::_in_context_for::domain_layer::entity::application_user::_new_for_context::email_sender_trait::EmailSenderTrait;
use crate::infrastructure_layer::error::base_error::base_error::BaseError;
use crate::infrastructure_layer::repository::data_provider::_in_context_for::domain_layer::entity::application_user_reset_password_token::_new_for_context::_in_context_for::_resource::redis::_new_for_context::base::Base as ApplicationUserResetPasswordTokenDataProviderRedis;
use crate::infrastructure_layer::service::_in_context_for::infrastructure_layer::repository::_new_for_context::aggregate_connection_pool::AggregateConnectionPool;
use crate::infrastructure_layer::service::_in_context_for::infrastructure_layer::repository::_new_for_context::connection_extractor::ConnectionExtractor;
use crate::infrastructure_layer::service::_in_context_for::domain_layer::entity::application_user::_new_for_context::email_sender::EmailSender;
use crate::presentation_layer::data_transfer_object::request::_in_context_for::presentation_layer::service::actix_web::request_handler::application_programming_interface::version_1::mobile::_in_context_for::domain_layer::entity::application_user::_new_for_context::authorization::_new_for_context::resend_email_for_reset_password::base::Base as Request;
use redis::Connection;
use std::sync::Arc;

pub struct Base;

impl Base {
    pub fn handle(
        aggregate_connection_pool: Arc<AggregateConnectionPool>,
        request: Request
    ) -> Result<(), BaseError> {     // TODO Защита от частого посыла емэй
        // let application_user_id: i64 = request.into_inner();

        // let connection: &'_ mut Connection = &mut *ConnectionExtractor::get_redis_connection(&aggregate_connection_pool)?;

        // if let Some(application_user_reset_password_token) = ApplicationUserResetPasswordTokenDataProviderRedis::find_by_application_user_id(
        //     connection, &application_user_id
        // )? 
        // {
        //     EmailSender::send_application_user_reset_password_token(&application_user_reset_password_token)?;
    
        //     return Ok(());
        // }

        // return Err(
        //     BaseError::EntityError {
        //         entity_error: EntityError::ApplicationUserResetPasswordTokenError {
        //             application_user_reset_password_token_error: ApplicationUserResetPasswordTokenError::NotFound
        //         }
        //     }
        // );

        // TODO Проверить логку, почему здесь нет проверки на существование пользователя. То есть, почему логика отличается от другиз резетов
        return Ok(());
    }
}