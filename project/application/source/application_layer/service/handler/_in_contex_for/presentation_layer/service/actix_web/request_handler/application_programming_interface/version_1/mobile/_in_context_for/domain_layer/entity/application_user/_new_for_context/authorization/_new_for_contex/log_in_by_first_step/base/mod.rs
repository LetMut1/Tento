use crate::domain_layer::entity::application_user_log_in_token::ApplicationUserLogInToken;
use crate::domain_layer::error::entity_error::_component::_in_context_for::domain_layer::entity::application_user::_new_for_context::application_user_error::ApplicationUserError;
use crate::domain_layer::error::entity_error::entity_error::EntityError;
use crate::domain_layer::repository::data_provider::_in_context_for::domain_layer::entity::application_user_log_in_token::_new_for_context::_in_context_for::_resource::redis::_new_for_context::base_trait::BaseTrait as ApplicationUserLogInTokenDataProviderRedisTrait;
use crate::domain_layer::repository::data_provider::_in_context_for::domain_layer::entity::application_user::_new_for_context::_in_context_for::_resource::postgresql::_new_for_context::base_trait::BaseTrait as ApplicationUserDataProviderPostgresqlTrait;
use crate::domain_layer::repository::state_manager::_in_context_for::domain_layer::entity::application_user_log_in_token::_new_for_context::_in_context_for::_resource::redis::_new_for_context::base_trait::BaseTrait as ApplicationUserLogInTokenStateManagerRedisTrait;
use crate::domain_layer::service::_in_context_for::domain_layer::entity::application_user_log_in_token::_new_for_context::value_generator_trait::ValueGeneratorTrait;
use crate::domain_layer::service::_in_context_for::domain_layer::entity::application_user::_new_for_context::email_sender_trait::EmailSenderTrait;
use crate::domain_layer::service::validator::_in_context_for::domain_layer::entity::application_user::_new_for_context::base_trait::BaseTrait as ApplicationUserValidatorTrait;
use crate::infrastructure_layer::error::base_error::_component::logic_error::LogicError;
use crate::infrastructure_layer::error::base_error::base_error::BaseError;
use crate::infrastructure_layer::repository::data_provider::_in_context_for::domain_layer::entity::application_user_log_in_token::_new_for_context::_in_context_for::_resource::redis::_new_for_context::base::Base as ApplicationUserLogInTokenDataProviderRedis;
use crate::infrastructure_layer::repository::data_provider::_in_context_for::domain_layer::entity::application_user::_new_for_context::_in_context_for::_resource::postgresql::_new_for_context::base::Base as ApplicationUserDataProviderPostgresql;
use crate::infrastructure_layer::repository::state_manager::_in_context_for::domain_layer::entity::application_user_log_in_token::_new_for_context::_in_context_for::_resource::redis::_new_for_context::base::Base as ApplicationUserLogInTokenStateManagerRedis;
use crate::infrastructure_layer::service::_in_context_for::domain_layer::entity::application_user_log_in_token::_new_for_context::value_generator::ValueGenerator;
use crate::infrastructure_layer::service::_in_context_for::domain_layer::entity::application_user::_new_for_context::email_sender::EmailSender;
use crate::infrastructure_layer::service::_in_context_for::infrastructure_layer::repository::_new_for_context::aggregate_connection_pool::AggregateConnectionPool;
use crate::infrastructure_layer::service::_in_context_for::infrastructure_layer::repository::_new_for_context::connection_extractor::ConnectionExtractor;
use crate::infrastructure_layer::service::validator::_in_context_for::domain_layer::entity::application_user::_new_for_context::base::Base as ApplicationUserValidator;
use crate::presentation_layer::data_transfer_object::request::_in_context_for::presentation_layer::service::actix_web::request_handler::application_programming_interface::version_1::mobile::_in_context_for::domain_layer::entity::application_user::_new_for_context::authorization::_new_for_context::log_in_by_first_step::base::Base as Request;
use crate::presentation_layer::data_transfer_object::response::_in_context_for::presentation_layer::service::actix_web::request_handler::application_programming_interface::version_1::mobile::_in_context_for::domain_layer::entity::application_user::_new_for_context::authorization::_new_for_context::log_in_by_first_step::base::Base as Response;
use redis::Connection;
use std::sync::Arc;

pub struct Base;

impl Base {
    pub fn handle(      // TODO Если два логина на разные устройства, и коды подтверждения еще не введены? То есть, приийдет пользоватею два разных кода, а оне не узнает, какой код к какому устройству
        aggregate_connection_pool: Arc<AggregateConnectionPool>,
        request: Request
    ) -> Result<Response, BaseError> {
        let (
            application_user_log_in_token_device_id, 
            application_user_email, 
            application_user_password
        ) : (
            String,
            String,
            String
        ) = request.into_inner();

        if let Some(application_user) = ApplicationUserDataProviderPostgresql::find_by_email(
            &mut *ConnectionExtractor::get_postgresql_connection(&aggregate_connection_pool)?, application_user_email.as_str()
        )? 
        {
            if ApplicationUserValidator::is_valid_password(application_user_password.as_str())
            && ApplicationUserValidator::is_valid_password_hash(application_user_password.as_str(), application_user.get_password_hash())? {
                let application_user_log_in_token: ApplicationUserLogInToken<'_>;

                let application_user_id: &'_ i64;
                match application_user.get_id() {
                    Some(application_user_id_) => {
                        application_user_id = application_user_id_;
                    },
                    None => {
                        return Err(BaseError::LogicError {logic_error: LogicError::new(false, "Application_user_id should exist")});
                    }
                }

                let redis_connection: &'_ mut Connection = &mut *ConnectionExtractor::get_redis_connection(&aggregate_connection_pool)?;

                match ApplicationUserLogInTokenDataProviderRedis::find_by_application_user_id_and_device_id(
                    redis_connection, application_user_id, application_user_log_in_token_device_id.as_str()
                )? 
                {
                    Some(application_user_log_in_token_) => {
                        application_user_log_in_token = application_user_log_in_token_;

                        ApplicationUserLogInTokenStateManagerRedis::update_expiration_time(redis_connection, &application_user_log_in_token)?;
                    },
                    None => {
                        application_user_log_in_token = ApplicationUserLogInToken::new(
                            application_user_id,
                            application_user_log_in_token_device_id.as_str(),
                            ValueGenerator::generate(),
                            0
                        );

                        ApplicationUserLogInTokenStateManagerRedis::create(redis_connection, &application_user_log_in_token)?;
                    }
                }

                EmailSender::send_application_user_log_in_token(
                    application_user_log_in_token.get_value(), application_user.get_email()
                )?;

                return Ok(Response::new(*application_user_id));
            }
            
            return Err(BaseError::EntityError {entity_error: EntityError::ApplicationUserError {application_user_error: ApplicationUserError::WrongPassword}});
        }

        return Err(BaseError::EntityError {entity_error: EntityError::ApplicationUserError {application_user_error: ApplicationUserError::NotFound}});
    }
}