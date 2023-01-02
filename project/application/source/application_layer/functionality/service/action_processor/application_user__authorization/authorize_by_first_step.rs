use crate::application_layer::data::action_processor_result::ActionProcessorResult;
use crate::application_layer::data::entity_workflow_exception::ApplicationUser_WorkflowException;
use crate::domain_layer::functionality::service::application_user__password_hash_resolver::ApplicationUser_PasswordHashResolver;
use crate::domain_layer::functionality::service::application_user__validator::ApplicationUser_Validator;
use crate::domain_layer::functionality::service::application_user_log_in_token__value_generator::ApplicationUserLogInToken_ValueGenerator;
use crate::infrastructure_layer::data::error_auditor::BacktracePart;
use crate::infrastructure_layer::data::error_auditor::BaseError;
use crate::infrastructure_layer::data::error_auditor::ErrorAuditor;
use crate::infrastructure_layer::data::error_auditor::ResourceError;
use crate::infrastructure_layer::data::error_auditor::RunTimeError;
use crate::infrastructure_layer::functionality::repository::application_user__postgresql_repository::ApplicationUser_PostgresqlRepository;
use crate::infrastructure_layer::functionality::repository::application_user_log_in_token__postgresql_repository::ApplicationUserLogInToken_PostgresqlRepository;
use crate::infrastructure_layer::functionality::repository::application_user_log_in_token__postgresql_repository::Insert;
use crate::infrastructure_layer::functionality::repository::application_user_log_in_token__postgresql_repository::Update;
use crate::infrastructure_layer::functionality::service::application_user__email_sender::ApplicationUser_EmailSender;
use crate::infrastructure_layer::functionality::service::environment_configuration_resolver::EnvironmentConfigurationResolver;
use extern_crate::bb8_postgres::PostgresConnectionManager as PostgresqlConnectionManager;
use extern_crate::bb8::Pool;
use extern_crate::serde::Deserialize;
use extern_crate::serde::Serialize;
use extern_crate::tokio_postgres::Socket;
use extern_crate::tokio_postgres::tls::MakeTlsConnect;
use extern_crate::tokio_postgres::tls::TlsConnect;
use std::clone::Clone;
use std::marker::Send;
use std::marker::Sync;

pub struct ActionProcessor;

impl ActionProcessor {
    pub async fn process<'a, T>(      // TODO Если два логина на разные устройства, и коды подтверждения еще не введены? То есть, приийдет пользоватею два разных кода, а оне не узнает, какой код к какому устройству
        environment_configuration_resolver: &'a EnvironmentConfigurationResolver,
        core_postgresql_connection_pool: Pool<PostgresqlConnectionManager<T>>,
        authorization_postgresql_connection_pool: Pool<PostgresqlConnectionManager<T>>,
        incoming: Incoming
    ) -> Result<ActionProcessorResult<Outcoming>, ErrorAuditor>
    where
        T: MakeTlsConnect<Socket> + Clone + Send + Sync + 'static,
        <T as MakeTlsConnect<Socket>>::Stream: Send + Sync,
        <T as MakeTlsConnect<Socket>>::TlsConnect: Send,
        <<T as MakeTlsConnect<Socket>>::TlsConnect as TlsConnect<Socket>>::Future: Send
    {
        if ApplicationUser_Validator::is_valid_password(incoming.application_user_password.as_str()) {
            match core_postgresql_connection_pool.get().await {
                Ok(core_postgresql_pooled_connection) => {
                    let core_postgresql_connection = &*core_postgresql_pooled_connection;

                    let application_user = match ApplicationUser_Validator::is_valid_email(incoming.application_user_email_or_application_user_nickname.as_str()) {
                        Ok(is_valid_email) => {
                            if is_valid_email {
                                match ApplicationUser_PostgresqlRepository::find_2(core_postgresql_connection, incoming.application_user_email_or_application_user_nickname).await {
                                    Ok(application_user_) => {
                                        match application_user_ {
                                            Some(application_user__) => application_user__,
                                            None => {
                                                return Ok(ActionProcessorResult::application_user__workflow_exception(ApplicationUser_WorkflowException::NotFound));
                                            }
                                        }
                                    }
                                    Err(mut error) => {
                                        error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                                        return Err(error);
                                    }
                                }
                            } else {
                                if ApplicationUser_Validator::is_valid_nickname(incoming.application_user_email_or_application_user_nickname.as_str()) {
                                    match ApplicationUser_PostgresqlRepository::find_1(core_postgresql_connection, incoming.application_user_email_or_application_user_nickname).await {
                                        Ok(application_user_) => {
                                            match application_user_ {
                                                Some(application_user__) => application_user__,
                                                None => {
                                                    return Ok(ActionProcessorResult::application_user__workflow_exception(ApplicationUser_WorkflowException::NotFound));
                                                }
                                            }
                                        }
                                        Err(mut error) => {
                                            error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                                            return Err(error);
                                        }
                                    }
                                } else {
                                    return Ok(ActionProcessorResult::application_user__workflow_exception(ApplicationUser_WorkflowException::InvalidNickname));
                                }
                            }
                        }
                        Err(mut error) => {
                            error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                            return Err(error);
                        }
                    };

                    match ApplicationUser_PasswordHashResolver::is_valid(incoming.application_user_password.as_str(), application_user.get_password_hash()) {
                        Ok(is_valid) => {
                            if is_valid {
                                let application_user_id = application_user.get_id();

                                match authorization_postgresql_connection_pool.get().await {
                                    Ok(authorization_postgresql_pooled_connection) => {
                                        let authorization_postgresql_connection = &*authorization_postgresql_pooled_connection;

                                        match ApplicationUserLogInToken_PostgresqlRepository::find_1(
                                            authorization_postgresql_connection, application_user_id, incoming.application_user_device_id.as_str()
                                        ).await {
                                            Ok(application_user_log_in_token_) => {
                                                let application_user_log_in_token = match application_user_log_in_token_ {
                                                    Some(mut application_user_log_in_token__) => {
                                                        if let Err(mut error) = ApplicationUserLogInToken_PostgresqlRepository::update(
                                                            authorization_postgresql_connection,
                                                            &mut application_user_log_in_token__,
                                                            Update { application_user_log_in_token_expires_at: true }
                                                        ).await {
                                                            error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                                                            return Err(error);
                                                        }

                                                        application_user_log_in_token__
                                                    }
                                                    None => {
                                                        let insert = Insert {
                                                            application_user_id,
                                                            application_user_device_id: incoming.application_user_device_id.as_str(),
                                                            application_user_log_in_token_value: ApplicationUserLogInToken_ValueGenerator::generate(),
                                                            application_user_log_in_token_wrong_enter_tries_quantity: 0
                                                        };

                                                        match ApplicationUserLogInToken_PostgresqlRepository::create(
                                                            authorization_postgresql_connection, insert
                                                        ).await {
                                                            Ok(application_user_log_in_token__) => application_user_log_in_token__,
                                                            Err(mut error) => {
                                                                error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                                                                return Err(error);
                                                            }
                                                        }
                                                    }
                                                };

                                                if let Err(mut error) = ApplicationUser_EmailSender::send_application_user_log_in_token(
                                                    environment_configuration_resolver, application_user_log_in_token.get_value(), application_user.get_email()
                                                ) {
                                                    error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                                                    return Err(error);
                                                }

                                                return Ok(ActionProcessorResult::outcoming(Outcoming { application_user_id }));
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
                                                BaseError::RunTimeError { run_time_error: RunTimeError::ResourceError { resource_error: ResourceError::ConnectionPoolPostgresqlError { bb8_postgresql_error: error } } },
                                                BacktracePart::new(line!(), file!(), None)
                                            )
                                        );
                                    }
                                }
                            }

                            return Ok(ActionProcessorResult::application_user__workflow_exception(ApplicationUser_WorkflowException::WrongPassword));
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
                            BaseError::RunTimeError { run_time_error: RunTimeError::ResourceError { resource_error: ResourceError::ConnectionPoolPostgresqlError { bb8_postgresql_error: error } } },
                            BacktracePart::new(line!(), file!(), None)
                        )
                    );
                }
            }
        }

        return Ok(ActionProcessorResult::application_user__workflow_exception(ApplicationUser_WorkflowException::InvalidPassword));
    }
}

#[cfg_attr(feature = "facilitate_non_automatic_functional_testing", derive(Serialize))]
#[derive(Deserialize)]
#[serde(crate = "extern_crate::serde")]
pub struct Incoming {
    application_user_device_id: String,
    application_user_email_or_application_user_nickname: String,
    application_user_password: String
}

#[cfg(not(feature = "facilitate_non_automatic_functional_testing"))]
#[derive(Serialize)]
#[serde(crate = "extern_crate::serde")]
struct Outcoming {
    application_user_id: i64
}

#[cfg(feature = "facilitate_non_automatic_functional_testing")]
#[derive(Serialize, Deserialize)]
#[serde(crate = "extern_crate::serde")]
pub struct Outcoming {
    application_user_id: i64
}