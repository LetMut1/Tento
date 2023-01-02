use crate::application_layer::data::action_processor_result::ActionProcessorResult;
use crate::application_layer::data::entity_workflow_exception::ApplicationUserRegistrationConfirmationToken_WorkflowException;
use crate::application_layer::data::entity_workflow_exception::ApplicationUser_WorkflowException;
use crate::domain_layer::data::entity::application_user_registration_confirmation_token::ApplicationUserRegistrationConfirmationToken;
use crate::domain_layer::functionality::service::application_user__validator::ApplicationUser_Validator;
use crate::domain_layer::functionality::service::application_user_registration_confirmation_token__expiration_time_resolver::ApplicationUserRegistrationConfirmationToken_ExpirationTimeResolver;
use crate::domain_layer::functionality::service::application_user_registration_confirmation_token__validator::ApplicationUserRegistrationConfirmationToken_Validator;
use crate::domain_layer::functionality::service::application_user_registration_confirmation_token__wrong_enter_tries_quantity_incrementor::ApplicationUserRegistrationConfirmationToken_WrongEnterTriesQuantityIncrementor;
use crate::infrastructure_layer::data::error_auditor::BacktracePart;
use crate::infrastructure_layer::data::error_auditor::BaseError;
use crate::infrastructure_layer::data::error_auditor::ErrorAuditor;
use crate::infrastructure_layer::data::error_auditor::ResourceError;
use crate::infrastructure_layer::data::error_auditor::RunTimeError;
use crate::infrastructure_layer::functionality::repository::application_user_registration_confirmation_token__postgresql_repository::ApplicationUserRegistrationConfirmationToken_PostgresqlRepository;
use crate::infrastructure_layer::functionality::repository::application_user_registration_confirmation_token__postgresql_repository::Update;
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
                    match ApplicationUserRegistrationConfirmationToken_Validator::is_valid_value(incoming.application_user_registration_token_value.as_str()) {
                        Ok(is_valid_value) => {
                            if is_valid_value {
                                match authorization_postgresql_connection_pool.get().await {
                                    Ok(authorization_postgresql_pooled_connection) => {
                                        let authorization_postgresql_connection = &*authorization_postgresql_pooled_connection;

                                        match ApplicationUserRegistrationConfirmationToken_PostgresqlRepository::find_1(
                                            authorization_postgresql_connection, incoming.application_user_email.as_str()
                                        ).await {
                                            Ok(application_user_registration_confirmation_token) => {
                                                if let Some(mut application_user_registration_confirmation_token_) = application_user_registration_confirmation_token {
                                                    if !ApplicationUserRegistrationConfirmationToken_ExpirationTimeResolver::is_expired(&application_user_registration_confirmation_token_) {
                                                        if !application_user_registration_confirmation_token_.get_is_approved() {
                                                            if application_user_registration_confirmation_token_.get_value().as_bytes() == incoming.application_user_registration_token_value.as_bytes() {
                                                                application_user_registration_confirmation_token_.set_is_approved(true);

                                                                if let Err(mut error) = ApplicationUserRegistrationConfirmationToken_PostgresqlRepository::update(
                                                                    authorization_postgresql_connection,
                                                                    &mut application_user_registration_confirmation_token_,
                                                                    Update { application_user_registration_confirmation_token_expires_at: false }
                                                                ).await {
                                                                    error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                                                                    return Err(error);
                                                                }

                                                                return Ok(ActionProcessorResult::outcoming(Outcoming { application_user_registration_token_is_approved: true }));
                                                            } else {
                                                                if let Err(mut error) = ApplicationUserRegistrationConfirmationToken_WrongEnterTriesQuantityIncrementor::increment(&mut application_user_registration_confirmation_token_) {
                                                                    error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                                                                    return Err(error);
                                                                }

                                                                if application_user_registration_confirmation_token_.get_wrong_enter_tries_quantity() <= ApplicationUserRegistrationConfirmationToken::WRONG_ENTER_TRIES_QUANTITY_LIMIT {
                                                                    if let Err(mut error) = ApplicationUserRegistrationConfirmationToken_PostgresqlRepository::update(
                                                                        authorization_postgresql_connection,
                                                                        &mut application_user_registration_confirmation_token_,
                                                                        Update { application_user_registration_confirmation_token_expires_at: false }
                                                                    ).await {
                                                                        error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                                                                        return Err(error);
                                                                    }
                                                                } else {
                                                                    if let Err(mut error) = ApplicationUserRegistrationConfirmationToken_PostgresqlRepository::delete(
                                                                        authorization_postgresql_connection, application_user_registration_confirmation_token_.get_application_user_email()
                                                                    ).await {
                                                                        error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                                                                        return Err(error);
                                                                    }
                                                                }

                                                                return Ok(ActionProcessorResult::outcoming(Outcoming { application_user_registration_token_is_approved: false }));
                                                            }
                                                        }

                                                        return Ok(ActionProcessorResult::application_user_registration_confirmation_token__workflow_exception(ApplicationUserRegistrationConfirmationToken_WorkflowException::AlreadyApproved));
                                                    }

                                                    return Ok(ActionProcessorResult::application_user_registration_confirmation_token__workflow_exception(ApplicationUserRegistrationConfirmationToken_WorkflowException::AlreadyExpired));
                                                }

                                                return Ok(ActionProcessorResult::application_user_registration_confirmation_token__workflow_exception(ApplicationUserRegistrationConfirmationToken_WorkflowException::NotFound));
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

                            return Ok(ActionProcessorResult::application_user_registration_confirmation_token__workflow_exception(ApplicationUserRegistrationConfirmationToken_WorkflowException::InvalidValue));
                        }
                        Err(mut error) => {
                            error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                            return Err(error);
                        }
                    }
                }

                return Ok(ActionProcessorResult::application_user__workflow_exception(ApplicationUser_WorkflowException::InvalidEmail));
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
    application_user_email: String,
    application_user_registration_token_value: String
}

#[cfg(not(feature = "facilitate_non_automatic_functional_testing"))]
#[derive(Serialize)]
#[serde(crate = "extern_crate::serde")]
struct Outcoming {
    application_user_registration_token_is_approved: bool
}

#[cfg(feature = "facilitate_non_automatic_functional_testing")]
#[derive(Serialize, Deserialize)]
#[serde(crate = "extern_crate::serde")]
pub struct Outcoming {
    application_user_registration_token_is_approved: bool
}