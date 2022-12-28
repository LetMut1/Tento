use crate::application_layer::data::action_processor_result::ActionProcessorResult;
use crate::application_layer::data::entity_workflow_exception::ApplicationUserResetPasswordToken_WorkflowException;
use crate::application_layer::data::entity_workflow_exception::ApplicationUser_WorkflowException;
use crate::infrastructure_layer::data::error_auditor::BacktracePart;
use crate::infrastructure_layer::data::error_auditor::BaseError;
use crate::infrastructure_layer::data::error_auditor::ErrorAuditor;
use crate::infrastructure_layer::data::error_auditor::ResourceError;
use crate::infrastructure_layer::data::error_auditor::RunTimeError;
use crate::infrastructure_layer::functionality::repository::application_user__postgresql_repository::ApplicationUser_PostgresqlRepository;
use crate::infrastructure_layer::functionality::repository::application_user_reset_password_token__postgresql_repository::ApplicationUserResetPasswordToken_PostgresqlRepository;
use crate::infrastructure_layer::functionality::service::application_user__email_sender::ApplicationUser_EmailSender;
use crate::infrastructure_layer::functionality::service::environment_configuration_resolver::EnvironmentConfigurationResolver;
use extern_crate::bb8_postgres::PostgresConnectionManager as PostgresqlConnectionManager;
use extern_crate::bb8::Pool;
use extern_crate::serde::Deserialize;
use crate::domain_layer::functionality::service::application_user_reset_password_token__expiration_time_resolver::ApplicationUserResetPasswordToken_ExpirationTimeResolver;
use extern_crate::tokio_postgres::Socket;
use extern_crate::tokio_postgres::tls::MakeTlsConnect;
use extern_crate::tokio_postgres::tls::TlsConnect;
use std::clone::Clone;
use std::marker::Send;
use std::marker::Sync;

#[cfg(feature = "facilitate_non_automatic_functional_testing")]
use extern_crate::serde::Serialize;

pub struct ActionProcessor;

impl ActionProcessor {
    pub async fn process<'a, T>(
        environment_configuration_resolver: &'a EnvironmentConfigurationResolver,
        core_postgresql_connection_pool: Pool<PostgresqlConnectionManager<T>>,
        authorization_postgresql_connection_pool: Pool<PostgresqlConnectionManager<T>>,
        incoming: Incoming
    ) -> Result<ActionProcessorResult<()>, ErrorAuditor>
    where
        T: MakeTlsConnect<Socket> + Clone + Send + Sync + 'static,
        <T as MakeTlsConnect<Socket>>::Stream: Send + Sync,
        <T as MakeTlsConnect<Socket>>::TlsConnect: Send,
        <<T as MakeTlsConnect<Socket>>::TlsConnect as TlsConnect<Socket>>::Future: Send
    {                                                           // TODO Защита от частого посыла емэй
        match authorization_postgresql_connection_pool.get().await {
            Ok(authorization_postgresql_pooled_connection) => {
                match ApplicationUserResetPasswordToken_PostgresqlRepository::find_1(
                    &*authorization_postgresql_pooled_connection, incoming.application_user_id
                ).await {
                    Ok(application_user_reset_password_token) => {
                        if let Some(application_user_reset_password_token_) = application_user_reset_password_token {
                            let is_expired = match ApplicationUserResetPasswordToken_ExpirationTimeResolver::is_expired(&application_user_reset_password_token_) {
                                Ok(is_expired_) => is_expired_,
                                Err(mut error) => {
                                    error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                                    return Err(error);
                                }
                            };
                            if !is_expired {
                                if !application_user_reset_password_token_.get_is_approved() {
                                    match core_postgresql_connection_pool.get().await {
                                        Ok(core_postgresql_pooled_connection) => {
                                            match ApplicationUser_PostgresqlRepository::find_3(
                                                &*core_postgresql_pooled_connection, incoming.application_user_id
                                            ).await {
                                                Ok(application_user) => {
                                                    if let Some(application_user_) = application_user {
                                                        if let Err(mut error) = ApplicationUser_EmailSender::send_application_user_reset_password_token(
                                                            environment_configuration_resolver, application_user_reset_password_token_.get_value(), application_user_.get_email()
                                                        ) {
                                                            error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                                                            return Err(error);
                                                        }

                                                        return Ok(ActionProcessorResult::outcoming(()));
                                                    }

                                                    return Ok(ActionProcessorResult::application_user__workflow_exception(ApplicationUser_WorkflowException::NotFound));
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

                                return Ok(ActionProcessorResult::application_user_reset_password_token__workflow_exception(ApplicationUserResetPasswordToken_WorkflowException::AlreadyApproved));
                            }

                            return Ok(ActionProcessorResult::application_user_reset_password_token__workflow_exception(ApplicationUserResetPasswordToken_WorkflowException::AlreadyExpired));
                        }

                        return Ok(ActionProcessorResult::application_user_reset_password_token__workflow_exception(ApplicationUserResetPasswordToken_WorkflowException::NotFound));
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
}

#[cfg_attr(feature = "facilitate_non_automatic_functional_testing", derive(Serialize))]
#[derive(Deserialize)]
#[serde(crate = "extern_crate::serde")]
pub struct Incoming {
    application_user_id: i64
}