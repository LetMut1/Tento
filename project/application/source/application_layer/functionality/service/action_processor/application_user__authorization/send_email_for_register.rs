use crate::application_layer::data::action_processor_result::ActionProcessorResult;
use crate::application_layer::data::entity_workflow_exception::ApplicationUserRegistrationConfirmationToken_WorkflowException;
use crate::application_layer::data::entity_workflow_exception::ApplicationUser_WorkflowException;
use crate::domain_layer::data::entity::application_user_registration_confirmation_token::ApplicationUserRegistrationConfirmationToken;
use crate::domain_layer::functionality::service::application_user__validator::ApplicationUser_Validator;
use crate::domain_layer::functionality::service::application_user_registration_confirmation_token__expiration_time_resolver::ApplicationUserRegistrationConfirmationToken_ExpirationTimeResolver;
use crate::infrastructure_layer::data::error_auditor::BacktracePart;
use crate::infrastructure_layer::data::error_auditor::BaseError;
use crate::infrastructure_layer::data::error_auditor::ErrorAuditor;
use crate::infrastructure_layer::data::error_auditor::ResourceError;
use crate::infrastructure_layer::data::error_auditor::RunTimeError;
use crate::infrastructure_layer::functionality::repository::application_user_registration_confirmation_token__postgresql_repository::ApplicationUserRegistrationConfirmationToken_PostgresqlRepository;
use crate::infrastructure_layer::functionality::service::application_user__email_sender::ApplicationUser_EmailSender;
use crate::infrastructure_layer::functionality::service::date_time_resolver::DateTimeResolver;
use crate::infrastructure_layer::functionality::service::environment_configuration_resolver::EnvironmentConfigurationResolver;
use extern_crate::bb8_postgres::PostgresConnectionManager as PostgresqlConnectionManager;
use extern_crate::bb8::Pool;
use extern_crate::serde::Deserialize;
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
        authorization_postgresql_connection_pool: Pool<PostgresqlConnectionManager<T>>,
        incoming: Incoming
    ) -> Result<ActionProcessorResult<()>, ErrorAuditor>
    where
        T: MakeTlsConnect<Socket> + Clone + Send + Sync + 'static,
        <T as MakeTlsConnect<Socket>>::Stream: Send + Sync,
        <T as MakeTlsConnect<Socket>>::TlsConnect: Send,
        <<T as MakeTlsConnect<Socket>>::TlsConnect as TlsConnect<Socket>>::Future: Send
    {                                                                               // TODO  TODO  TODO  TODO сделать На Редисе механизм для невозможности почстоянно отравки емэйла. (Сохранять, если отправлено, и проверять, что отпрпавили. удалять по времени)
        match ApplicationUser_Validator::is_valid_email(incoming.application_user_email.as_str()) {
            Ok(is_valid_email) => {
                if is_valid_email {
                    match authorization_postgresql_connection_pool.get().await {
                        Ok(authorization_postgresql_pooled_connection) => {
                            let authorization_postgresql_connection = &*authorization_postgresql_pooled_connection;

                            match ApplicationUserRegistrationConfirmationToken_PostgresqlRepository::find_1(
                                authorization_postgresql_connection, incoming.application_user_email.as_str()
                            ).await {
                                Ok(application_user_registration_confirmation_token) => {
                                    if let Some(mut application_user_registration_confirmation_token_) = application_user_registration_confirmation_token {
                                        let is_expired = match ApplicationUserRegistrationConfirmationToken_ExpirationTimeResolver::is_expired(&application_user_registration_confirmation_token_) {
                                            Ok(is_expired_) => is_expired_,
                                            Err(mut error) => {
                                                error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                                                return Err(error);
                                            }
                                        };
                                        if !is_expired {
                                            if !application_user_registration_confirmation_token_.get_is_approved() {
                                                let expires_at = match DateTimeResolver::add_interval_from_now_formated(ApplicationUserRegistrationConfirmationToken::QUANTITY_OF_MINUTES_FOR_EXPIRATION as i64) {
                                                    Ok(expires_at_) => expires_at_,
                                                    Err(mut error) => {
                                                        error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                                                        return Err(error);
                                                    }
                                                };

                                                application_user_registration_confirmation_token_.set_expires_at(expires_at);

                                                if let Err(mut error) = ApplicationUserRegistrationConfirmationToken_PostgresqlRepository::update(
                                                    authorization_postgresql_connection, &application_user_registration_confirmation_token_
                                                ).await {
                                                    error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                                                    return Err(error);
                                                }

                                                if let Err(mut error) = ApplicationUser_EmailSender::send_application_user_registration_confirmation_token(
                                                    environment_configuration_resolver,
                                                    application_user_registration_confirmation_token_.get_value(),
                                                    incoming.application_user_email.as_str()
                                                ) {
                                                    error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                                                    return Err(error);
                                                }

                                                return Ok(ActionProcessorResult::new_with_outcoming(()));
                                            }

                                            return Ok(ActionProcessorResult::new_with_application_user_registration_confirmation_token_workflow_exception(ApplicationUserRegistrationConfirmationToken_WorkflowException::AlreadyApproved));
                                        }

                                        return Ok(ActionProcessorResult::new_with_application_user_registration_confirmation_token_workflow_exception(ApplicationUserRegistrationConfirmationToken_WorkflowException::AlreadyExpired));
                                    }

                                    return Ok(ActionProcessorResult::new_with_application_user_registration_confirmation_token_workflow_exception(ApplicationUserRegistrationConfirmationToken_WorkflowException::NotFound));
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

#[cfg_attr(feature="facilitate_non_automatic_functional_testing", derive(Serialize))]
#[derive(Deserialize)]
#[serde(crate = "extern_crate::serde")]
pub struct Incoming {
    application_user_email: String
}