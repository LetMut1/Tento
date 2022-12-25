use crate::application_layer::data::action_handler_result::ActionHandlerResult;
use crate::application_layer::data::entity_workflow_exception::ApplicationUserResetPasswordToken_WorkflowException;
use crate::domain_layer::data::entity::application_user_reset_password_token::ApplicationUserResetPasswordToken;
use crate::domain_layer::functionality::service::application_user_reset_password_token__expiration_time_resolver::ApplicationUserResetPasswordToken_ExpirationTimeResolver;
use crate::domain_layer::functionality::service::application_user_reset_password_token__validator::ApplicationUserResetPasswordToken_Validator;
use crate::domain_layer::functionality::service::application_user_reset_password_token__wrong_enter_tries_quantity_incrementor::ApplicationUserResetPasswordToken_WrongEnterTriesQuantityIncrementor;
use crate::infrastructure_layer::data::error_auditor::BacktracePart;
use crate::infrastructure_layer::data::error_auditor::BaseError;
use crate::infrastructure_layer::data::error_auditor::ErrorAuditor;
use crate::infrastructure_layer::data::error_auditor::ResourceError;
use crate::infrastructure_layer::data::error_auditor::RunTimeError;
use crate::infrastructure_layer::functionality::repository::application_user_reset_password_token__postgresql_repository::ApplicationUserResetPasswordToken_PostgresqlRepository;
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
    pub async fn process<T>(
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
            application_user_id,
            application_user_reset_password_token_value
        ) = incoming.into_inner();

        match ApplicationUserResetPasswordToken_Validator::is_valid_value(application_user_reset_password_token_value.as_str()) {
            Ok(is_valid_value) => {
                if is_valid_value {
                    match authorization_postgresql_connection_pool.get().await {
                        Ok(authorization_postgresql_pooled_connection) => {
                            let authorization_postgresql_connection = &*authorization_postgresql_pooled_connection;

                            match ApplicationUserResetPasswordToken_PostgresqlRepository::find_1(
                                authorization_postgresql_connection, application_user_id
                            ).await {
                                Ok(application_user_reset_password_token) => {
                                    if let Some(mut application_user_reset_password_token_) = application_user_reset_password_token {
                                        let is_expired = match ApplicationUserResetPasswordToken_ExpirationTimeResolver::is_expired(&application_user_reset_password_token_) {
                                            Ok(is_expired_) => is_expired_,
                                            Err(mut error) => {
                                                error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                                                return Err(error);
                                            }
                                        };
                                        if !is_expired {
                                            if !application_user_reset_password_token_.get_is_approved() {
                                                if application_user_reset_password_token_.get_value().as_bytes() == application_user_reset_password_token_value.as_bytes() {
                                                    application_user_reset_password_token_.set_is_approved(true);

                                                    if let Err(mut error) = ApplicationUserResetPasswordToken_PostgresqlRepository::update(
                                                        authorization_postgresql_connection, &application_user_reset_password_token_
                                                    ).await {
                                                        error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                                                        return Err(error);
                                                    }

                                                    return Ok(ActionHandlerResult::new_with_outcoming(Outcoming::new(true)));
                                                } else {
                                                    if let Err(mut error) = ApplicationUserResetPasswordToken_WrongEnterTriesQuantityIncrementor::increment(&mut application_user_reset_password_token_) {
                                                        error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                                                        return Err(error);
                                                    }

                                                    if application_user_reset_password_token_.get_wrong_enter_tries_quantity() <= ApplicationUserResetPasswordToken::WRONG_ENTER_TRIES_QUANTITY_LIMIT {
                                                        if let Err(mut error) = ApplicationUserResetPasswordToken_PostgresqlRepository::update(
                                                            authorization_postgresql_connection, &application_user_reset_password_token_
                                                        ).await {
                                                            error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                                                            return Err(error);
                                                        }
                                                    } else {
                                                        if let Err(mut error) = ApplicationUserResetPasswordToken_PostgresqlRepository::delete(
                                                            authorization_postgresql_connection, application_user_reset_password_token_.get_application_user_id()
                                                        ).await {
                                                            error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                                                            return Err(error);
                                                        }
                                                    }

                                                    return Ok(ActionHandlerResult::new_with_outcoming(Outcoming::new(false)));
                                                }
                                            }

                                            return Ok(ActionHandlerResult::new_with_application_user_reset_password_token_workflow_exception(ApplicationUserResetPasswordToken_WorkflowException::AlreadyApproved));
                                        }

                                        return Ok(ActionHandlerResult::new_with_application_user_reset_password_token_workflow_exception(ApplicationUserResetPasswordToken_WorkflowException::AlreadyExpired));
                                    }

                                    return Ok(ActionHandlerResult::new_with_application_user_reset_password_token_workflow_exception(ApplicationUserResetPasswordToken_WorkflowException::NotFound));
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

                return Ok(ActionHandlerResult::new_with_application_user_reset_password_token_workflow_exception(ApplicationUserResetPasswordToken_WorkflowException::InvalidValue));
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
    application_user_id: i64,
    application_user_reset_password_token_value: String
}

impl Incoming {
    pub fn into_inner(
        self
    ) -> (i64, String) {
        return (
            self.application_user_id,
            self.application_user_reset_password_token_value
        );
    }
}

#[cfg_attr(feature="facilitate_non_automatic_functional_testing", derive(Deserialize))]
#[derive(Serialize)]
#[serde(crate = "extern_crate::serde")]
pub struct Outcoming {
    application_user_reset_password_token_is_approved: bool
}

impl Outcoming {
    pub fn new(
        application_user_reset_password_token_is_approved: bool
    ) -> Self {
        return Self {
            application_user_reset_password_token_is_approved
        };
    }
}