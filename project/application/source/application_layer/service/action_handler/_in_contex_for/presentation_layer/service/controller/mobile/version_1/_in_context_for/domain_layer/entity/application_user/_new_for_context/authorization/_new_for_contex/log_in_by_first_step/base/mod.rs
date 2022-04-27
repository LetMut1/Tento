use bb8_postgres::PostgresConnectionManager as PostgresqlConnectionManager;
use bb8_redis::RedisConnectionManager;
use bb8::Pool;
use crate::domain_layer::entity::application_user_log_in_token::ApplicationUserLogInToken;
use crate::domain_layer::entity::application_user::ApplicationUser;
use crate::domain_layer::error::entity_error::_component::_in_context_for::domain_layer::entity::application_user::_new_for_context::application_user_error::ApplicationUserError;
use crate::domain_layer::error::entity_error::entity_error::EntityError;
use crate::domain_layer::service::_in_context_for::domain_layer::entity::application_user_log_in_token::_new_for_context::value_generator_trait::ValueGeneratorTrait;
use crate::domain_layer::service::_in_context_for::domain_layer::entity::application_user::_new_for_context::email_sender_trait::EmailSenderTrait;
use crate::domain_layer::service::_in_context_for::domain_layer::entity::application_user::_new_for_context::password_hash_resolver_trait::PasswordHashResolverTrait;
use crate::domain_layer::service::validator::_in_context_for::domain_layer::entity::application_user::_new_for_context::base_trait::BaseTrait as ApplicationUserValidatorTrait;
use crate::infrastructure_layer::error::error_auditor::_component::error_aggregator::_component::logic_error::LogicError;
use crate::infrastructure_layer::error::error_auditor::_component::error_aggregator::_component::run_time_error::_component::resource_error::resource_error::ResourceError;
use crate::infrastructure_layer::error::error_auditor::_component::error_aggregator::_component::run_time_error::run_time_error::RunTimeError;
use crate::infrastructure_layer::error::error_auditor::_component::error_aggregator::error_aggregator::ErrorAggregator;
use crate::infrastructure_layer::error::error_auditor::_component::simple_backtrace::_component::backtrace_part::BacktracePart;
use crate::infrastructure_layer::error::error_auditor::error_auditor::ErrorAuditor;
use crate::infrastructure_layer::repository::data_provider::_in_context_for::domain_layer::entity::application_user_log_in_token::_new_for_context::_in_context_for::_resource::redis::_new_for_context::base::Base as ApplicationUserLogInTokenDataProviderRedis;
use crate::infrastructure_layer::repository::data_provider::_in_context_for::domain_layer::entity::application_user::_new_for_context::_in_context_for::_resource::postgresql::_new_for_context::base::Base as ApplicationUserDataProviderPostgresql;
use crate::infrastructure_layer::repository::state_manager::_in_context_for::domain_layer::entity::application_user_log_in_token::_new_for_context::_in_context_for::_resource::redis::_new_for_context::base::Base as ApplicationUserLogInTokenStateManagerRedis;
use crate::infrastructure_layer::service::_in_context_for::domain_layer::entity::application_user_log_in_token::_new_for_context::value_generator::ValueGenerator;
use crate::infrastructure_layer::service::_in_context_for::domain_layer::entity::application_user::_new_for_context::email_sender::EmailSender;
use crate::infrastructure_layer::service::_in_context_for::domain_layer::entity::application_user::_new_for_context::password_hash_resolver::PasswordHashResolver;
use crate::infrastructure_layer::service::validator::_in_context_for::domain_layer::entity::application_user::_new_for_context::base::Base as ApplicationUserValidator;
use crate::presentation_layer::data_transfer_object::request_data::_in_context_for::presentation_layer::service::controller::mobile::version_1::_in_context_for::domain_layer::entity::application_user::_new_for_context::authorization::_new_for_context::log_in_by_first_step::base::Base as RequestData;
use crate::presentation_layer::data_transfer_object::response_data::_in_context_for::presentation_layer::service::controller::mobile::version_1::_in_context_for::domain_layer::entity::application_user::_new_for_context::authorization::_new_for_context::log_in_by_first_step::base::Base as ResponseData;
use tokio_postgres::NoTls;

pub struct Base;

impl Base {
    pub async fn handle(      // TODO Если два логина на разные устройства, и коды подтверждения еще не введены? То есть, приийдет пользоватею два разных кода, а оне не узнает, какой код к какому устройству
        postgresql_connection_pool: Pool<PostgresqlConnectionManager<NoTls>>,
        redis_connection_pool: Pool<RedisConnectionManager>,
        request_data: RequestData
    ) -> Result<ResponseData, ErrorAuditor> {
        let (
            application_user_log_in_token_device_id, 
            application_user_email_or_application_user_nickname, 
            application_user_password
        ) = request_data.into_inner();

        if ApplicationUserValidator::is_valid_password(application_user_password.as_str()) {
            match postgresql_connection_pool.get().await {
                Ok(mut postgresql_pooled_connection) => {
                    let postgresql_connection = &mut *postgresql_pooled_connection;

                    let application_user: ApplicationUser;
                    match ApplicationUserValidator::is_valid_email(application_user_email_or_application_user_nickname.as_str()) {
                        Ok(is_valid_email) => {
                            if is_valid_email {
                                match ApplicationUserDataProviderPostgresql::find_by_email(postgresql_connection, application_user_email_or_application_user_nickname.as_str()).await? {
                                    Some(application_user_) => {
                                        application_user = application_user_;
                                    }
                                    None => {
                                        return Err(
                                            ErrorAuditor::new(
                                                ErrorAggregator::EntityError {entity_error: EntityError::ApplicationUserError {application_user_error: ApplicationUserError::NotFound}},
                                                BacktracePart::new(line!(), file!(), None)
                                            )
                                        );
                                    }
                                }
                            } else {
                                if ApplicationUserValidator::is_valid_nickname(application_user_email_or_application_user_nickname.as_str()) {
                                    match ApplicationUserDataProviderPostgresql::find_by_nickname(postgresql_connection, application_user_email_or_application_user_nickname.as_str()).await? {
                                        Some(application_user_) => {
                                            application_user = application_user_;
                                        }
                                        None => {
                                            return Err(
                                                ErrorAuditor::new(
                                                    ErrorAggregator::EntityError {entity_error: EntityError::ApplicationUserError {application_user_error: ApplicationUserError::NotFound}},
                                                    BacktracePart::new(line!(), file!(), None)
                                                )
                                            );
                                        }
                                    }
                                } else {
                                    return Err(
                                        ErrorAuditor::new(
                                            ErrorAggregator::EntityError {entity_error: EntityError::ApplicationUserError {application_user_error: ApplicationUserError::InvalidNickname}},
                                            BacktracePart::new(line!(), file!(), None)
                                        )
                                    );
                                }
                            }
                        }
                        Err(mut error) => {
                            error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));
            
                            return Err(error);
                        }
                    }

                    match PasswordHashResolver::is_valid(application_user_password.as_str(), application_user.get_password_hash()) {
                        Ok(is_valid) => {
                            if is_valid {
                                let application_user_log_in_token: ApplicationUserLogInToken<'_>;
        
                                let application_user_id: &'_ i64;
                                match application_user.get_id() {
                                    Some(application_user_id_) => {
                                        application_user_id = application_user_id_;
                                    }
                                    None => {
                                        return Err(
                                            ErrorAuditor::new(
                                                ErrorAggregator::LogicError {logic_error: LogicError::new(false, "Application_user_id should exist")},
                                                BacktracePart::new(line!(), file!(), None)
                                            )
                                        );
                                    }
                                }
        
                                match redis_connection_pool.get().await {
                                    Ok(mut redis_pooled_connection) => {
                                        let redis_connection = &mut *redis_pooled_connection;
        
                                        match ApplicationUserLogInTokenDataProviderRedis::find_by_application_user_id_and_device_id(
                                            redis_connection, application_user_id, application_user_log_in_token_device_id.as_str()
                                        ).await? {
                                            Some(application_user_log_in_token_) => {
                                                application_user_log_in_token = application_user_log_in_token_;
        
                                                ApplicationUserLogInTokenStateManagerRedis::update_expiration_time(redis_connection, &application_user_log_in_token).await?;
                                            }
                                            None => {
                                                application_user_log_in_token = ApplicationUserLogInToken::new(
                                                    application_user_id,
                                                    application_user_log_in_token_device_id.as_str(),
                                                    ValueGenerator::generate(),
                                                    0
                                                );
        
                                                ApplicationUserLogInTokenStateManagerRedis::create(redis_connection, &application_user_log_in_token).await?;
                                            }
                                        }
        
                                        if let Err(mut error) = EmailSender::send_application_user_log_in_token(
                                            application_user_log_in_token.get_value(), application_user.get_email()
                                        ) {
                                            error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));
                            
                                            return Err(error);
                                        }
        
                                        return Ok(ResponseData::new(*application_user_id));
                                    }
                                    Err(error) => {
                                        return Err(
                                            ErrorAuditor::new(
                                                ErrorAggregator::RunTimeError {run_time_error: RunTimeError::ResourceError {resource_error: ResourceError::ConnectionPoolRedisError {bb8_redis_error: error}}},
                                                BacktracePart::new(line!(), file!(), None)
                                            )
                                        );
                                    }
                                }
                            }
        
                            return Err(
                                ErrorAuditor::new(
                                    ErrorAggregator::EntityError {entity_error: EntityError::ApplicationUserError {application_user_error: ApplicationUserError::WrongPassword}},
                                    BacktracePart::new(line!(), file!(), None)
                                )
                            );
                        }
                        Err(mut error) => {
                            error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));
            
                            return Err(error);
                        }
                    }
                }
                Err(error) => {
                    return Err(
                        ErrorAuditor::new(
                            ErrorAggregator::RunTimeError {run_time_error: RunTimeError::ResourceError {resource_error: ResourceError::ConnectionPoolPostgresqlError {bb8_postgresql_error: error}}},
                            BacktracePart::new(line!(), file!(), None)
                        )
                    );
                }
            }
        }

        return Err(
            ErrorAuditor::new(
                ErrorAggregator::EntityError {entity_error: EntityError::ApplicationUserError {application_user_error: ApplicationUserError::InvalidPassword}},
                BacktracePart::new(line!(), file!(), None)
            )
        );
    }
}