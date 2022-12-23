use crate::application_layer::data::action_handler_result::ActionHandlerResult;
use crate::application_layer::data::entity_workflow_exception::_component::_in_context_for::domain_layer::data::entity::application_user_registration_confirmation_token::_new_for_context::application_user_registration_confirmation_token_workflow_exception::ApplicationUserRegistrationConfirmationTokenWorkflowException;
use crate::application_layer::data::entity_workflow_exception::_component::_in_context_for::domain_layer::data::entity::application_user::_new_for_context::application_user_workflow_exception::ApplicationUserWorkflowException;
use crate::domain_layer::data::entity::application_user_registration_confirmation_token::ApplicationUserRegistrationConfirmationToken;
use crate::domain_layer::functionality::service::application_user__validator::ApplicationUser_Validator;
use crate::domain_layer::functionality::service::application_user_registration_confirmation_token__expiration_time_resolver::ApplicationUserRegistrationConfirmationToken_ExpirationTimeResolver;
use crate::domain_layer::functionality::service::application_user_registration_confirmation_token__validator::ApplicationUserRegistrationConfirmationToken_Validator;
use crate::domain_layer::functionality::service::application_user_registration_confirmation_token__wrong_enter_tries_quantity_incrementor::ApplicationUserRegistrationConfirmationToken_WrongEnterTriesQuantityIncrementor;
use crate::infrastructure_layer::data::error_auditor::_component::base_error::_component::run_time_error::_component::resource_error::resource_error::ResourceError;
use crate::infrastructure_layer::data::error_auditor::_component::base_error::_component::run_time_error::run_time_error::RunTimeError;
use crate::infrastructure_layer::data::error_auditor::_component::base_error::base_error::BaseError;
use crate::infrastructure_layer::data::error_auditor::_component::simple_backtrace::_component::backtrace_part::BacktracePart;
use crate::infrastructure_layer::data::error_auditor::error_auditor::ErrorAuditor;
use crate::infrastructure_layer::functionality::repository::application_user_registration_confirmation_token__postgresql_repository::ApplicationUserRegistrationConfirmationToken_PostgresqlRepository;
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

pub struct Base;

impl Base {
    pub async fn handle<'a, T>(
        authorization_postgresql_connection_pool: Pool<PostgresqlConnectionManager<T>>,
        incoming: Incoming
    ) -> Result<ActionHandlerResult<Outcoming>, ErrorAuditor>
    where
        T: MakeTlsConnect<Socket> + Clone + Send + Sync + 'static,
        <T as MakeTlsConnect<Socket>>::Stream: Send + Sync,
        <T as MakeTlsConnect<Socket>>::TlsConnect: Send,
        <<T as MakeTlsConnect<Socket>>::TlsConnect as TlsConnect<Socket>>::Future: Send
    {
        let (
            application_user_email,
            application_user_registration_confirmation_token_value
        ) = incoming.into_inner();

        match ApplicationUser_Validator::is_valid_email(application_user_email.as_str()) {
            Ok(is_valid_email) => {
                if is_valid_email {
                    match ApplicationUserRegistrationConfirmationToken_Validator::is_valid_value(application_user_registration_confirmation_token_value.as_str()) {
                        Ok(is_valid_value) => {
                            if is_valid_value {
                                match authorization_postgresql_connection_pool.get().await {
                                    Ok(authorization_postgresql_pooled_connection) => {
                                        let authorization_postgresql_connection = &*authorization_postgresql_pooled_connection;

                                        match ApplicationUserRegistrationConfirmationToken_PostgresqlRepository::find_1(
                                            authorization_postgresql_connection, application_user_email.as_str()
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
                                                            if application_user_registration_confirmation_token_.get_value().as_bytes() == application_user_registration_confirmation_token_value.as_bytes() {
                                                                application_user_registration_confirmation_token_.set_is_approved(true);

                                                                if let Err(mut error) = ApplicationUserRegistrationConfirmationToken_PostgresqlRepository::update(
                                                                    authorization_postgresql_connection, &application_user_registration_confirmation_token_
                                                                ).await {
                                                                    error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                                                                    return Err(error);
                                                                }

                                                                return Ok(ActionHandlerResult::new_with_outcoming(Outcoming::new(true)));
                                                            } else {
                                                                if let Err(mut error) = ApplicationUserRegistrationConfirmationToken_WrongEnterTriesQuantityIncrementor::increment(&mut application_user_registration_confirmation_token_) {
                                                                    error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                                                                    return Err(error);
                                                                }

                                                                if application_user_registration_confirmation_token_.get_wrong_enter_tries_quantity() <= ApplicationUserRegistrationConfirmationToken::WRONG_ENTER_TRIES_QUANTITY_LIMIT {
                                                                    if let Err(mut error) = ApplicationUserRegistrationConfirmationToken_PostgresqlRepository::update(
                                                                        authorization_postgresql_connection, &application_user_registration_confirmation_token_
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

                                                                return Ok(ActionHandlerResult::new_with_outcoming(Outcoming::new(false)));
                                                            }
                                                        }

                                                        return Ok(ActionHandlerResult::new_with_application_user_registration_confirmation_token_workflow_exception(ApplicationUserRegistrationConfirmationTokenWorkflowException::AlreadyApproved));
                                                    }

                                                    return Ok(ActionHandlerResult::new_with_application_user_registration_confirmation_token_workflow_exception(ApplicationUserRegistrationConfirmationTokenWorkflowException::AlreadyExpired));
                                                }

                                                return Ok(ActionHandlerResult::new_with_application_user_registration_confirmation_token_workflow_exception(ApplicationUserRegistrationConfirmationTokenWorkflowException::NotFound));
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

                            return Ok(ActionHandlerResult::new_with_application_user_registration_confirmation_token_workflow_exception(ApplicationUserRegistrationConfirmationTokenWorkflowException::InvalidValue));
                        }
                        Err(mut error) => {
                            error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                            return Err(error);
                        }
                    }
                }

                return Ok(ActionHandlerResult::new_with_application_user_workflow_exception(ApplicationUserWorkflowException::InvalidEmail));
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
    application_user_email: String,
    application_user_registration_confirmation_token_value: String
}

impl Incoming {
    pub fn into_inner(
        self
    ) -> (String, String) {
        return (
            self.application_user_email,
            self.application_user_registration_confirmation_token_value
        );
    }
}

#[cfg_attr(feature="facilitate_non_automatic_functional_testing", derive(Deserialize))]
#[derive(Serialize)]
#[serde(crate = "extern_crate::serde")]
pub struct Outcoming {
    application_user_registration_confirmation_token_is_approved: bool
}

impl Outcoming {
    pub fn new(
        application_user_registration_confirmation_token_is_approved: bool
    ) -> Self {
        return Self {
            application_user_registration_confirmation_token_is_approved
        };
    }
}