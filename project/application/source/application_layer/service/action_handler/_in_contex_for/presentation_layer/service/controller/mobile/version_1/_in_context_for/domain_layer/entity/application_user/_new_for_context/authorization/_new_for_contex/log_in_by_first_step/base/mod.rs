use bb8_postgres::PostgresConnectionManager as PostgresqlConnectionManager;
use bb8_redis::RedisConnectionManager;
use bb8::Pool;
use crate::application_layer::data_transfer_object::action_handler_incoming_data::_in_context_for::application_layer::service::action_handler::_in_context_for::presentation_layer::service::controller::mobile::version_1::_in_context_for::domain_layer::entity::application_user::_new_for_context::authorization::_new_for_context::log_in_by_first_step::base::_new_for_context::base::Base as ActionHandlerIncomingData;
use crate::application_layer::data_transfer_object::action_handler_outcoming_data::_in_context_for::application_layer::service::action_handler::_in_context_for::presentation_layer::service::controller::mobile::version_1::_in_context_for::domain_layer::entity::application_user::_new_for_context::authorization::_new_for_context::log_in_by_first_step::base::_new_for_context::base::Base as ActionHandlerOutcomingData;
use crate::application_layer::data_transfer_object::_in_context_for::application_layer::service::action_handler::_new_for_context::action_handler_result::ActionHandlerResult;
use crate::application_layer::data_transfer_object::_in_context_for::application_layer::service::action_handler::_new_for_context::entity_workflow_exception::_component::_in_context_for::domain_layer::entity::application_user::_new_for_context::application_user_workflow_exception::ApplicationUserWorkflowException;
use crate::domain_layer::entity::application_user_log_in_token::ApplicationUserLogInToken;
use crate::domain_layer::entity::application_user::ApplicationUser;
use crate::domain_layer::service::_in_context_for::domain_layer::entity::application_user_log_in_token::_new_for_context::value_generator::ValueGenerator;
use crate::domain_layer::service::_in_context_for::domain_layer::entity::application_user::_new_for_context::password_hash_resolver::PasswordHashResolver;
use crate::domain_layer::service::validator::_in_context_for::domain_layer::entity::application_user::_new_for_context::base::Base as ApplicationUserValidator;
use crate::infrastructure_layer::error::error_auditor::_component::error_aggregator::_component::logic_error::LogicError;
use crate::infrastructure_layer::error::error_auditor::_component::error_aggregator::_component::run_time_error::_component::resource_error::resource_error::ResourceError;
use crate::infrastructure_layer::error::error_auditor::_component::error_aggregator::_component::run_time_error::run_time_error::RunTimeError;
use crate::infrastructure_layer::error::error_auditor::_component::error_aggregator::error_aggregator::ErrorAggregator;
use crate::infrastructure_layer::error::error_auditor::_component::simple_backtrace::_component::backtrace_part::BacktracePart;
use crate::infrastructure_layer::error::error_auditor::error_auditor::ErrorAuditor;
use crate::infrastructure_layer::repository::data_provider::_in_context_for::domain_layer::entity::application_user_log_in_token::_new_for_context::_in_context_for::_resource::redis::_new_for_context::base::Base as ApplicationUserLogInTokenDataProviderRedis;
use crate::infrastructure_layer::repository::data_provider::_in_context_for::domain_layer::entity::application_user::_new_for_context::_in_context_for::_resource::postgresql::_new_for_context::base::Base as ApplicationUserDataProviderPostgresql;
use crate::infrastructure_layer::repository::state_manager::_in_context_for::domain_layer::entity::application_user_log_in_token::_new_for_context::_in_context_for::_resource::redis::_new_for_context::base::Base as ApplicationUserLogInTokenStateManagerRedis;
use crate::infrastructure_layer::service::_in_context_for::domain_layer::entity::application_user::_new_for_context::email_sender::EmailSender;
use crate::infrastructure_layer::service::environment_configuration_resolver::EnvironmentConfigurationResolver;
use std::clone::Clone;
use std::marker::Send;
use std::marker::Sync;
use tokio_postgres::Socket;
use tokio_postgres::tls::MakeTlsConnect;
use tokio_postgres::tls::TlsConnect;

pub struct Base;

impl Base {
    pub async fn handle<'a, T>(      // TODO Если два логина на разные устройства, и коды подтверждения еще не введены? То есть, приийдет пользоватею два разных кода, а оне не узнает, какой код к какому устройству
        environment_configuration_resolver: &'a EnvironmentConfigurationResolver,
        postgresql_connection_pool: Pool<PostgresqlConnectionManager<T>>,
        redis_connection_pool: Pool<RedisConnectionManager>,
        action_handler_incoming_data: ActionHandlerIncomingData
    ) -> Result<ActionHandlerResult<ActionHandlerOutcomingData>, ErrorAuditor>
    where
        T: MakeTlsConnect<Socket> + Clone + Send + Sync + 'static,
        <T as MakeTlsConnect<Socket>>::Stream: Send + Sync,
        <T as MakeTlsConnect<Socket>>::TlsConnect: Send,
        <<T as MakeTlsConnect<Socket>>::TlsConnect as TlsConnect<Socket>>::Future: Send
    {
        let (
            application_user_log_in_token_device_id, 
            application_user_email_or_application_user_nickname, 
            application_user_password
        ) = action_handler_incoming_data.into_inner();

        if ApplicationUserValidator::is_valid_password(application_user_password.as_str()) {
            match postgresql_connection_pool.get().await {
                Ok(mut postgresql_pooled_connection) => {
                    let postgresql_connection = &mut *postgresql_pooled_connection;

                    let application_user: ApplicationUser;
                    match ApplicationUserValidator::is_valid_email(application_user_email_or_application_user_nickname.as_str()) {
                        Ok(is_valid_email) => {
                            if is_valid_email {
                                match ApplicationUserDataProviderPostgresql::find_by_email(postgresql_connection, application_user_email_or_application_user_nickname.as_str()).await {
                                    Ok(application_user_) => {
                                        match application_user_ {
                                            Some(application_user__) => {
                                                application_user = application_user__;
                                            }
                                            None => {
                                                return Ok(ActionHandlerResult::new_with_application_user_workflow_exception(ApplicationUserWorkflowException::NotFound));
                                            }
                                        }
                                    }
                                    Err(mut error) => {
                                        error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));
                        
                                        return Err(error);
                                    }
                                }
                            } else {
                                if ApplicationUserValidator::is_valid_nickname(application_user_email_or_application_user_nickname.as_str()) {
                                    match ApplicationUserDataProviderPostgresql::find_by_nickname(postgresql_connection, application_user_email_or_application_user_nickname.as_str()).await {
                                        Ok(application_user_) => {
                                            match application_user_ {
                                                Some(application_user__) => {
                                                    application_user = application_user__;
                                                }
                                                None => {
                                                    return Ok(ActionHandlerResult::new_with_application_user_workflow_exception(ApplicationUserWorkflowException::NotFound));
                                                }
                                            }
                                        }
                                        Err(mut error) => {
                                            error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));
                            
                                            return Err(error);
                                        }
                                    }
                                } else {
                                    return Ok(ActionHandlerResult::new_with_application_user_workflow_exception(ApplicationUserWorkflowException::InvalidNickname));
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
        
                                let application_user_id: i64;
                                match application_user.get_id() {
                                    Some(application_user_id_) => {
                                        application_user_id = application_user_id_;
                                    }
                                    None => {
                                        return Err(
                                            ErrorAuditor::new(
                                                ErrorAggregator::LogicError { logic_error: LogicError::new(false, "Application_user_id should exist") },
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
                                        ).await {
                                            Ok(application_user_log_in_token_) => {
                                                match application_user_log_in_token_ {
                                                    Some(application_user_log_in_token__) => {
                                                        application_user_log_in_token = application_user_log_in_token__;
                
                                                        if let Err(mut error) = ApplicationUserLogInTokenStateManagerRedis::update_expiration_time(redis_connection, &application_user_log_in_token).await {
                                                            error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));
            
                                                            return Err(error);
                                                        }
                                                    }
                                                    None => {
                                                        application_user_log_in_token = ApplicationUserLogInToken::new(
                                                            application_user_id,
                                                            application_user_log_in_token_device_id.as_str(),
                                                            ValueGenerator::generate(),
                                                            0
                                                        );
                
                                                        if let Err(mut error) = ApplicationUserLogInTokenStateManagerRedis::create(redis_connection, &application_user_log_in_token).await {
                                                            error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));
            
                                                            return Err(error);
                                                        }
                                                    }
                                                }
                
                                                if let Err(mut error) = EmailSender::send_application_user_log_in_token(
                                                    environment_configuration_resolver, application_user_log_in_token.get_value(), application_user.get_email()
                                                ) {
                                                    error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));
                                    
                                                    return Err(error);
                                                }
                
                                                return Ok(ActionHandlerResult::new_with_action_handler_outcoming_data(ActionHandlerOutcomingData::new(application_user_id)));
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
                                                ErrorAggregator::RunTimeError { run_time_error: RunTimeError::ResourceError { resource_error: ResourceError::ConnectionPoolRedisError { bb8_redis_error: error } } },
                                                BacktracePart::new(line!(), file!(), None)
                                            )
                                        );
                                    }
                                }
                            }
        
                            return Ok(ActionHandlerResult::new_with_application_user_workflow_exception(ApplicationUserWorkflowException::WrongPassword));
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
                            ErrorAggregator::RunTimeError { run_time_error: RunTimeError::ResourceError { resource_error: ResourceError::ConnectionPoolPostgresqlError { bb8_postgresql_error: error } } },
                            BacktracePart::new(line!(), file!(), None)
                        )
                    );
                }
            }
        }

        return Ok(ActionHandlerResult::new_with_application_user_workflow_exception(ApplicationUserWorkflowException::InvalidPassword));
    }
}