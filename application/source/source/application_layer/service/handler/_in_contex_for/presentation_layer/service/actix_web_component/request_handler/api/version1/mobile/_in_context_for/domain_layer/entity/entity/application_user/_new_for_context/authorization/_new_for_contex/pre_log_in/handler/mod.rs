use crate::domain_layer::entity::entity::application_user_log_in_token::_core::device_id::DeviceId as ApplicationUserLogInTokenDeviceId;
use crate::domain_layer::entity::entity::application_user_log_in_token::application_user_log_in_token::ApplicationUserLogInToken;
use crate::domain_layer::entity::entity::application_user::_core::email::Email;
use crate::domain_layer::entity::entity::application_user::_core::password::Password;
use crate::domain_layer::error::entity_error::_core::_in_context_for::domain_layer::entity::entity::application_user::_new_for_context::application_user_error::ApplicationUserError;
use crate::domain_layer::error::entity_error::entity_error::EntityError;
use crate::domain_layer::repository::_in_context_for::domain_layer::entity::entity::application_user_log_in_token::_new_for_context::_in_context_for::_resource::redis::_new_for_context::base_repository_trait::BaseRepositoryTrait as ApplicationUserLogInTokenBaseRepositoryTrait;
use crate::domain_layer::repository::_in_context_for::domain_layer::entity::entity::application_user::_new_for_context::_in_context_for::_resource::postgresql::_new_for_context::base_repository_trait::BaseRepositoryTrait as ApplicationUserBaseRepositoryTrait;
use crate::domain_layer::service::_in_context_for::domain_layer::entity::entity::application_user::_new_for_context::email_sender_trait::EmailSenderTrait;
use crate::domain_layer::service::_in_context_for::domain_layer::entity::entity::application_user::_new_for_context::password_hash_resolver_trait::PasswordHashResolverTrait;
use crate::infrastructure_layer::error::base_error::base_error::BaseError;
use crate::infrastructure_layer::repository::_in_context_for::domain_layer::entity::entity::application_user_log_in_token::_new_for_context::_in_context_for::_resource::redis::_new_for_context::base_repository::BaseRepository as ApplicationUserLogInTokenBaseRepository;
use crate::infrastructure_layer::repository::_in_context_for::domain_layer::entity::entity::application_user::_new_for_context::_in_context_for::_resource::postgresql::_new_for_context::base_repository::BaseRepository as ApplicationUserBaseRepository;
use crate::infrastructure_layer::service::_in_context_for::_resource::_new_for_context::aggregate_connection_pool::AggregateConnectionPool;
use crate::infrastructure_layer::service::_in_context_for::_resource::_new_for_context::connection_extractor::ConnectionExtractor;
use crate::infrastructure_layer::service::_in_context_for::domain_layer::entity::entity::application_user::_new_for_context::email_sender::EmailSender;
use crate::infrastructure_layer::service::_in_context_for::domain_layer::entity::entity::application_user::_new_for_context::password_hash_resolver::PasswordHashResolver;
use crate::presentation_layer::data_transfer_object::request::_in_context_for::presentation_layer::service::actix_web_component::request_handler::api::version1::mobile::_in_context_for::domain_layer::entity::entity::application_user::_new_for_context::authorization::_new_for_context::pre_log_in::request::Request;
use crate::presentation_layer::data_transfer_object::response::_in_context_for::presentation_layer::service::actix_web_component::request_handler::api::version1::mobile::_in_context_for::domain_layer::entity::entity::application_user::_new_for_context::authorization::_new_for_context::pre_log_in::response::Response;
use redis::Connection;
use std::sync::Arc;

pub struct Handler;

impl Handler {
    pub fn handle(aggregate_connection_pool: Arc<AggregateConnectionPool>, request: Request) -> Result<Response, BaseError> {
        let (
            application_user_log_in_token_device_id, 
            application_user_email, 
            application_user_password
        ) = request.into_inner();

        let application_user_log_in_token_device_id: ApplicationUserLogInTokenDeviceId = ApplicationUserLogInTokenDeviceId::new_from_string(
            application_user_log_in_token_device_id
        )?;

        if let Some(application_user) = ApplicationUserBaseRepository::get_by_email(
            &*ConnectionExtractor::get_postgresql_connection(&aggregate_connection_pool)?, &Email::new(application_user_email)
        )? 
        {
            if PasswordHashResolver::is_valid(&Password::new(application_user_password), application_user.get_password_hash())? {
                let application_user_log_in_token: ApplicationUserLogInToken<'_>;

                let connection: &'_ mut Connection = &mut *ConnectionExtractor::get_redis_connection(&aggregate_connection_pool)?;

                match ApplicationUserLogInTokenBaseRepository::get_by_application_user_id_and_device_id(
                    connection, application_user.get_id()?, &application_user_log_in_token_device_id
                )? 
                {
                    Some(existing_application_user_log_in_token) => {
                        application_user_log_in_token = existing_application_user_log_in_token;

                        ApplicationUserLogInTokenBaseRepository::update_expiration_time(connection, &application_user_log_in_token)?;
                    },
                    None => {
                        application_user_log_in_token = ApplicationUserLogInToken::new(&application_user, &application_user_log_in_token_device_id)?;

                        ApplicationUserLogInTokenBaseRepository::create(connection, &application_user_log_in_token)?;
                    }
                }

                EmailSender::send_application_user_log_in_token(&application_user_log_in_token)?;

                return Ok(Response::new(application_user.get_id()?.get_value()));
            }
            
            return Err(BaseError::EntityError(EntityError::ApplicationUserError(ApplicationUserError::WrongPassword)));
        }

        return Err(BaseError::EntityError(EntityError::ApplicationUserError(ApplicationUserError::NotFound)));
    }
}