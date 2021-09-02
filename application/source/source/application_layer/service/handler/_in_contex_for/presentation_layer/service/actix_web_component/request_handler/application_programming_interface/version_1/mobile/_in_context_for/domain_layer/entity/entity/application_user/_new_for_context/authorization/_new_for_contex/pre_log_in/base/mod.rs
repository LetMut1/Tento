use crate::domain_layer::entity::entity::application_user_log_in_token::_component::device_id::DeviceId as ApplicationUserLogInTokenDeviceId;
use crate::domain_layer::entity::entity::application_user_log_in_token::application_user_log_in_token::ApplicationUserLogInToken;
use crate::domain_layer::entity::entity::application_user::_component::email::Email;
use crate::domain_layer::entity::entity::application_user::_component::password::Password;
use crate::domain_layer::error::entity_error::_component::_in_context_for::domain_layer::entity::entity::application_user::_new_for_context::application_user_error::ApplicationUserError;
use crate::domain_layer::error::entity_error::entity_error::EntityError;
use crate::domain_layer::repository::data_provider::_in_context_for::domain_layer::entity::entity::application_user_log_in_token::_new_for_context::_in_context_for::_resource::redis::_new_for_context::base_trait::BaseTrait as DataProviderApplicationUserLogInTokenRedisTrait;
use crate::domain_layer::repository::data_provider::_in_context_for::domain_layer::entity::entity::application_user::_new_for_context::_in_context_for::_resource::postgresql::_new_for_context::base_trait::BaseTrait as DataProviderApplicationUserPostgresqlTrait;
use crate::domain_layer::repository::state_manager::_in_context_for::domain_layer::entity::entity::application_user_log_in_token::_new_for_context::_in_context_for::_resource::redis::_new_for_context::base_trait::BaseTrait as StateManagerApplicationUserLogInTokenRedisTrait;
use crate::domain_layer::service::_in_context_for::domain_layer::entity::entity::application_user::_new_for_context::email_sender_trait::EmailSenderTrait;
use crate::domain_layer::service::_in_context_for::domain_layer::entity::entity::application_user::_new_for_context::password_hash_resolver_trait::PasswordHashResolverTrait;
use crate::domain_layer::service::factory::_in_context_for::domain_layer::entity::entity::application_user_log_in_token::_new_for_context::base::Base as ApplicationUserLogInTokenFactory;
use crate::infrastructure_layer::error::base_error::base_error::BaseError;
use crate::infrastructure_layer::repository::data_provider::_in_context_for::domain_layer::entity::entity::application_user_log_in_token::_new_for_context::_in_context_for::_resource::redis::_new_for_context::base::Base as DataProviderApplicationUserLogInTokenRedis;
use crate::infrastructure_layer::repository::data_provider::_in_context_for::domain_layer::entity::entity::application_user::_new_for_context::_in_context_for::_resource::postgresql::_new_for_context::base::Base as DataProviderApplicationUserPostgresql;
use crate::infrastructure_layer::repository::state_manager::_in_context_for::domain_layer::entity::entity::application_user_log_in_token::_new_for_context::_in_context_for::_resource::redis::_new_for_context::base::Base as StateManagerApplicationUserLogInTokenRedis;
use crate::infrastructure_layer::service::_in_context_for::infrastructure_layer::repository::_new_for_context::aggregate_connection_pool::AggregateConnectionPool;
use crate::infrastructure_layer::service::_in_context_for::infrastructure_layer::repository::_new_for_context::connection_extractor::ConnectionExtractor;
use crate::infrastructure_layer::service::_in_context_for::domain_layer::entity::entity::application_user::_new_for_context::email_sender::EmailSender;
use crate::infrastructure_layer::service::_in_context_for::domain_layer::entity::entity::application_user::_new_for_context::password_hash_resolver::PasswordHashResolver;
use crate::presentation_layer::data_transfer_object::request::_in_context_for::presentation_layer::service::actix_web_component::request_handler::application_programming_interface::version_1::mobile::_in_context_for::domain_layer::entity::entity::application_user::_new_for_context::authorization::_new_for_context::pre_log_in::base::Base as Request;
use crate::presentation_layer::data_transfer_object::response::_in_context_for::presentation_layer::service::actix_web_component::request_handler::application_programming_interface::version_1::mobile::_in_context_for::domain_layer::entity::entity::application_user::_new_for_context::authorization::_new_for_context::pre_log_in::base::Base as Response;
use redis::Connection;
use std::sync::Arc;

pub struct Base;

impl Base {
    pub fn handle(aggregate_connection_pool: Arc<AggregateConnectionPool>, request: Request) -> Result<Response, BaseError> {
        let (
            application_user_log_in_token_device_id, 
            application_user_email, 
            application_user_password
        ) : (
            String,
            String,
            String
        ) = request.into_inner();

        let application_user_log_in_token_device_id: ApplicationUserLogInTokenDeviceId = ApplicationUserLogInTokenDeviceId::new_from_string(
            application_user_log_in_token_device_id
        )?;

        if let Some(application_user) = DataProviderApplicationUserPostgresql::get_by_email(
            &*ConnectionExtractor::get_postgresqlxxxdelete_connection(&aggregate_connection_pool)?, &Email::new(application_user_email)
        )? 
        {
            if PasswordHashResolver::is_valid(&Password::new(application_user_password), application_user.get_password_hash())? {
                let application_user_log_in_token: ApplicationUserLogInToken<'_>;

                let connection: &'_ mut Connection = &mut *ConnectionExtractor::get_redis_connection(&aggregate_connection_pool)?;

                match DataProviderApplicationUserLogInTokenRedis::get_by_application_user_id_and_device_id(
                    connection, application_user.get_id()?, &application_user_log_in_token_device_id
                )? 
                {
                    Some(existing_application_user_log_in_token) => {
                        application_user_log_in_token = existing_application_user_log_in_token;

                        StateManagerApplicationUserLogInTokenRedis::update_expiration_time(connection, &application_user_log_in_token)?;
                    },
                    None => {
                        application_user_log_in_token = ApplicationUserLogInTokenFactory::new_from_application_user(
                            &application_user, &application_user_log_in_token_device_id
                        )?;

                        StateManagerApplicationUserLogInTokenRedis::create(connection, &application_user_log_in_token)?;
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