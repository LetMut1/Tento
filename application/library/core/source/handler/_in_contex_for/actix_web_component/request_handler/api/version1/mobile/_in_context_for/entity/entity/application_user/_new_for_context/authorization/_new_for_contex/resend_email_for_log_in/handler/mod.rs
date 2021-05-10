use crate::data_transfer_object::request_parameters::_in_context_for::actix_web_component::request_handler::api::version1::mobile::_in_context_for::entity::entity::application_user::_new_for_context::authorization::_new_for_context::resend_email_for_log_in::request::Request;
use crate::entity::entity::application_user_log_in_token::core::device_id::DeviceId as ApplicationUserLogInTokenDeviceId;
use crate::entity::entity::application_user::core::id::Id as ApplicationUserId;
use crate::error::main_error_kind::core::entity_error_kind::core::_in_context_for::entity::entity::application_user_log_in_token::_new_for_context::application_user_log_in_token::ApplicationUserLogInTokenErrorKind;
use crate::error::main_error_kind::core::entity_error_kind::entity_error_kind::EntityErrorKind;
use crate::error::main_error_kind::main_error_kind::MainErrorKind;
use crate::repository::_in_context_for::entity::entity::application_user_log_in_token::_new_for_context::_in_context_for::_resource::redis::_new_for_context::base_repository::BaseRepository as ApplicationUserLogInTokenBaseRepository;
use crate::service::_in_context_for::entity::entity::application_user::_new_for_context::email_sender::EmailSender;
use crate::utility::_in_context_for::_resource::_new_for_context::aggregate_connection_pool::AggregateConnectionPool;
use crate::utility::_in_context_for::_resource::_new_for_context::connection_extractor::ConnectionExtractor;
use std::sync::Arc;

pub struct Handler;

impl Handler {
    pub fn handle(request: Request, aggregate_connection_pool: Arc<AggregateConnectionPool>) -> Result<(), MainErrorKind> { // TODO сделать На Редисе механизм для невозможности почстоянно отравки емэйла. (Сохранять, если отправлено, и проверять, что отпрпавили. удалять по времени)
        let application_user_id: ApplicationUserId = ApplicationUserId::new_from_string(request.application_user_id)?;

        let application_user_log_in_token_device_id: ApplicationUserLogInTokenDeviceId =
        ApplicationUserLogInTokenDeviceId::new_from_string(request.application_user_log_in_token_device_id)?;

        if let Some(application_user_log_in_token) = 
        ApplicationUserLogInTokenBaseRepository::get_by_application_user_id_and_device_id(
            &mut *ConnectionExtractor::get_redis_connection(&aggregate_connection_pool)?, &application_user_id, &application_user_log_in_token_device_id
        )? 
        {
            EmailSender::send_application_user_log_in_token(&application_user_log_in_token)?;

            return Ok(());
        }

        return Err(MainErrorKind::EntityErrorKind(EntityErrorKind::ApplicationUserLogInTokenErrorKind(ApplicationUserLogInTokenErrorKind::NotFound)));
    }
}