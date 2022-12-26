use crate::application_layer::data::action_processor_result::ActionProcessorResult;
use crate::application_layer::data::entity_workflow_exception::ApplicationUser_WorkflowException;
use crate::domain_layer::data::entity::application_user_reset_password_token::ApplicationUserResetPasswordToken;
use crate::domain_layer::functionality::service::application_user__validator::ApplicationUser_Validator;
use crate::domain_layer::functionality::service::application_user_reset_password_token__expiration_time_resolver::ApplicationUserResetPasswordToken_ExpirationTimeResolver;
use crate::domain_layer::functionality::service::application_user_reset_password_token__value_generator::ApplicationUserResetPasswordToken_ValueGenerator;
use crate::infrastructure_layer::data::error_auditor::BacktracePart;
use crate::infrastructure_layer::data::error_auditor::BaseError;
use crate::infrastructure_layer::data::error_auditor::ErrorAuditor;
use crate::infrastructure_layer::data::error_auditor::ResourceError;
use crate::infrastructure_layer::data::error_auditor::RunTimeError;
use crate::infrastructure_layer::functionality::repository::application_user__postgresql_repository::ApplicationUser_PostgresqlRepository;
use crate::infrastructure_layer::functionality::repository::application_user_reset_password_token__postgresql_repository::ApplicationUserResetPasswordToken_PostgresqlRepository;
use crate::infrastructure_layer::functionality::repository::application_user_reset_password_token__postgresql_repository::Insert;
use crate::infrastructure_layer::functionality::service::application_user__email_sender::ApplicationUser_EmailSender;
use crate::infrastructure_layer::functionality::service::date_time_resolver::DateTimeResolver;
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
    pub async fn process<'a, T>(
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
        match ApplicationUser_Validator::is_valid_email(incoming.application_user_email.as_str()) {
            Ok(is_valid_email) => {
                if is_valid_email {
                    match core_postgresql_connection_pool.get().await {
                        Ok(core_postgresql_pooled_connection) => {
                            match ApplicationUser_PostgresqlRepository::find_2(
                                &*core_postgresql_pooled_connection, incoming.application_user_email
                            ).await {
                                Ok(application_user) => {
                                    if let Some(application_user_) = application_user {
                                        let application_user_id = application_user_.get_id();

                                        match authorization_postgresql_connection_pool.get().await {
                                            Ok(authorization_postgresql_pooled_connection) => {
                                                let authorization_postgresql_connection = &*authorization_postgresql_pooled_connection;

                                                match ApplicationUserResetPasswordToken_PostgresqlRepository::find_1(
                                                    authorization_postgresql_connection, application_user_id
                                                ).await {
                                                    Ok(application_user_reset_password_token_) => {
                                                        let application_user_reset_password_token = match application_user_reset_password_token_ {
                                                            Some(mut application_user_reset_password_token__) => {
                                                                let expires_at = match DateTimeResolver::add_interval_from_now_formated(ApplicationUserResetPasswordToken::QUANTITY_OF_MINUTES_FOR_EXPIRATION as i64) {
                                                                    Ok(expires_at_) => expires_at_,
                                                                    Err(mut error) => {
                                                                        error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                                                                        return Err(error);
                                                                    }
                                                                };

                                                                let is_expired = match ApplicationUserResetPasswordToken_ExpirationTimeResolver::is_expired(&application_user_reset_password_token__) {
                                                                    Ok(is_expired_) => is_expired_,
                                                                    Err(mut error) => {
                                                                        error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                                                                        return Err(error);
                                                                    }
                                                                };

                                                                application_user_reset_password_token__.set_expires_at(expires_at);
                                                                if is_expired || application_user_reset_password_token__.get_is_approved() {
                                                                    application_user_reset_password_token__
                                                                        .set_value(ApplicationUserResetPasswordToken_ValueGenerator::generate())
                                                                        .set_wrong_enter_tries_quantity(0)
                                                                        .set_is_approved(false);
                                                                }

                                                                if let Err(mut error) = ApplicationUserResetPasswordToken_PostgresqlRepository::update(
                                                                    authorization_postgresql_connection, &application_user_reset_password_token__
                                                                ).await {
                                                                    error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                                                                    return Err(error);
                                                                }

                                                                application_user_reset_password_token__
                                                            }
                                                            None => {
                                                                let insert = Insert::new(
                                                                    application_user_id,
                                                                    ApplicationUserResetPasswordToken_ValueGenerator::generate(),
                                                                    0,
                                                                    false
                                                                );

                                                                match ApplicationUserResetPasswordToken_PostgresqlRepository::create(
                                                                    authorization_postgresql_connection, insert
                                                                ).await {
                                                                    Ok(application_user_reset_password_token__) => application_user_reset_password_token__,
                                                                    Err(mut error) => {
                                                                        error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                                                                        return Err(error);
                                                                    }
                                                                }
                                                            }
                                                        };

                                                        if let Err(mut error) = ApplicationUser_EmailSender::send_application_user_reset_password_token(
                                                            environment_configuration_resolver, application_user_reset_password_token.get_value(), application_user_.get_email()
                                                        ) {
                                                            error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                                                            return Err(error);
                                                        }

                                                        return Ok(ActionProcessorResult::new_with_outcoming(Outcoming { application_user_id }));
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

                                    return Ok(ActionProcessorResult::new_with_application_user_workflow_exception(ApplicationUser_WorkflowException::NotFound));
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

                return Ok(ActionProcessorResult::new_with_application_user_workflow_exception(ApplicationUser_WorkflowException::InvalidEmail));
            }
            Err(mut error) => {
                error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                return Err(error);
            }
        }
    }
}

#[cfg_attr(feature = "facilitate_non_automatic_functional_testing", derive(Serialize))]
#[derive(Deserialize)]
#[serde(crate = "extern_crate::serde")]
pub struct Incoming {
    application_user_email: String
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