use crate::application_layer::data::action_handler_result::ActionHandlerResult;
use crate::application_layer::data::entity_workflow_exception::_component::_in_context_for::domain_layer::data::entity::application_user_reset_password_token::_new_for_context::application_user_reset_password_token_workflow_exception::ApplicationUserResetPasswordTokenWorkflowException;
use crate::application_layer::data::entity_workflow_exception::_component::_in_context_for::domain_layer::data::entity::application_user::_new_for_context::application_user_workflow_exception::ApplicationUserWorkflowException;
use crate::domain_layer::data::entity::application_user_reset_password_token::ApplicationUserResetPasswordToken;
use crate::domain_layer::functionality::service::application_user__password_hash_resolver::ApplicationUser_PasswordHashResolver;
use crate::domain_layer::functionality::service::application_user__validator::ApplicationUser_Validator;
use crate::domain_layer::functionality::service::application_user_reset_password_token__expiration_time_resolver::ApplicationUserResetPasswordToken_ExpirationTimeResolver;
use crate::domain_layer::functionality::service::application_user_reset_password_token__wrong_enter_tries_quantity_incrementor::ApplicationUserResetPasswordToken_WrongEnterTriesQuantityIncrementor;
use crate::domain_layer::functionality::service::validator::_in_context_for::domain_layer::data::entity::application_user_reset_password_token::_new_for_context::base::Base as ApplicationUserResetPasswordTokenValidator;
use crate::infrastructure_layer::data::error_auditor::_component::base_error::_component::run_time_error::_component::resource_error::resource_error::ResourceError;
use crate::infrastructure_layer::data::error_auditor::_component::base_error::_component::run_time_error::run_time_error::RunTimeError;
use crate::infrastructure_layer::data::error_auditor::_component::base_error::base_error::BaseError;
use crate::infrastructure_layer::data::error_auditor::_component::simple_backtrace::_component::backtrace_part::BacktracePart;
use crate::infrastructure_layer::data::error_auditor::error_auditor::ErrorAuditor;
use crate::infrastructure_layer::functionality::repository::data_provider::_in_context_for::domain_layer::data::entity::application_user_reset_password_token::_new_for_context::_in_context_for::_resource::postgresql::_new_for_context::base::Base as ApplicationUserResetPasswordTokenDataProviderPostgresql;
use crate::infrastructure_layer::functionality::repository::data_provider::_in_context_for::domain_layer::data::entity::application_user::_new_for_context::_in_context_for::_resource::postgresql::_new_for_context::base::Base as ApplicationUserDataProviderPostgresql;
use crate::infrastructure_layer::functionality::repository::state_manager::_in_context_for::domain_layer::data::entity::application_user_reset_password_token::_new_for_context::_in_context_for::_resource::postgresql::_new_for_context::base::Base as ApplicationUserResetPasswordTokenStateManagerPostgresql;
use crate::infrastructure_layer::functionality::repository::state_manager::_in_context_for::domain_layer::data::entity::application_user::_new_for_context::_in_context_for::_resource::postgresql::_new_for_context::base::Base as ApplicationUserStateManagerPostgresql;
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

pub struct Base;

impl Base {
    pub async fn handle<T>(
        core_postgresql_connection_pool: Pool<PostgresqlConnectionManager<T>>,
        authorization_postgresql_connection_pool: Pool<PostgresqlConnectionManager<T>>,
        incoming: Incoming
    ) -> Result<ActionHandlerResult<()>, ErrorAuditor>
    where
        T: MakeTlsConnect<Socket> + Clone + Send + Sync + 'static,
        <T as MakeTlsConnect<Socket>>::Stream: Send + Sync,
        <T as MakeTlsConnect<Socket>>::TlsConnect: Send,
        <<T as MakeTlsConnect<Socket>>::TlsConnect as TlsConnect<Socket>>::Future: Send
    { // TODO  !!!!! Это ресет для пользователя, забывшего пароль. НО также нужно сделать АККУРАТНО ресетпассворд для залогиневшегося пользователя с повторной отправкой старого пароля !!!!!!!!!
        let (
            application_user_id,
            application_user_password,
            application_user_reset_password_token_value
        ) = incoming.into_inner();

        match ApplicationUserResetPasswordTokenValidator::is_valid_value(application_user_reset_password_token_value.as_str()) {
            Ok(is_valid_value) => {
                if is_valid_value {
                    if ApplicationUser_Validator::is_valid_password(application_user_password.as_str()) {
                        match authorization_postgresql_connection_pool.get().await {
                            Ok(authorization_postgresql_pooled_connection) => {
                                let authorization_postgresql_connection = &*authorization_postgresql_pooled_connection;

                                match ApplicationUserResetPasswordTokenDataProviderPostgresql::find_1(
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
                                                if application_user_reset_password_token_.get_is_approved() {
                                                    if application_user_reset_password_token_.get_value() == application_user_reset_password_token_value.as_str() {
                                                        match core_postgresql_connection_pool.get().await {
                                                            Ok(core_postgresql_pooled_connection) => {
                                                                let core_postgresql_connection = &*core_postgresql_pooled_connection;

                                                                match ApplicationUserDataProviderPostgresql::find_3(core_postgresql_connection, application_user_id).await {
                                                                    Ok(application_user) => {
                                                                        if let Some(mut application_user_) = application_user {
                                                                            match ApplicationUser_PasswordHashResolver::create(application_user_password.as_str()) {
                                                                                Ok(password_hash) => {
                                                                                    application_user_.set_password_hash(password_hash);

                                                                                    if let Err(mut error) = ApplicationUserStateManagerPostgresql::update(core_postgresql_connection, &application_user_).await {
                                                                                        error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                                                                                        return Err(error);
                                                                                    }

                                                                                    if let Err(mut error) = ApplicationUserResetPasswordTokenStateManagerPostgresql::delete(
                                                                                        authorization_postgresql_connection, application_user_reset_password_token_.get_application_user_id()
                                                                                    ).await {
                                                                                        error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                                                                                        return Err(error);
                                                                                    }

                                                                                    return Ok(ActionHandlerResult::new_with_outcoming(()));
                                                                                }
                                                                                Err(mut error) => {
                                                                                    error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                                                                                    return Err(error);
                                                                                }
                                                                            }
                                                                        }

                                                                        return Ok(ActionHandlerResult::new_with_application_user_workflow_exception(ApplicationUserWorkflowException::NotFound));
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

                                                    if let Err(mut error) = ApplicationUserResetPasswordToken_WrongEnterTriesQuantityIncrementor::increment(&mut application_user_reset_password_token_) {
                                                        error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                                                        return Err(error);
                                                    }

                                                    if application_user_reset_password_token_.get_wrong_enter_tries_quantity() <= ApplicationUserResetPasswordToken::WRONG_ENTER_TRIES_QUANTITY_LIMIT {
                                                        if let Err(mut error) = ApplicationUserResetPasswordTokenStateManagerPostgresql::update(
                                                            authorization_postgresql_connection, &application_user_reset_password_token_
                                                        ).await {
                                                            error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                                                            return Err(error);
                                                        }
                                                    } else {
                                                        if let Err(mut error) = ApplicationUserResetPasswordTokenStateManagerPostgresql::delete(
                                                            authorization_postgresql_connection, application_user_reset_password_token_.get_application_user_id()
                                                        ).await {
                                                            error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                                                            return Err(error);
                                                        }
                                                    }

                                                    return Ok(ActionHandlerResult::new_with_application_user_reset_password_token_workflow_exception(ApplicationUserResetPasswordTokenWorkflowException::WrongValue));
                                                }

                                                return Ok(ActionHandlerResult::new_with_application_user_reset_password_token_workflow_exception(ApplicationUserResetPasswordTokenWorkflowException::IsNotApproved));
                                            }

                                            return Ok(ActionHandlerResult::new_with_application_user_reset_password_token_workflow_exception(ApplicationUserResetPasswordTokenWorkflowException::AlreadyExpired));
                                        }

                                        return Ok(ActionHandlerResult::new_with_application_user_reset_password_token_workflow_exception(ApplicationUserResetPasswordTokenWorkflowException::NotFound));
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

                    return Ok(ActionHandlerResult::new_with_application_user_workflow_exception(ApplicationUserWorkflowException::InvalidPassword));
                }

                return Ok(ActionHandlerResult::new_with_application_user_reset_password_token_workflow_exception(ApplicationUserResetPasswordTokenWorkflowException::InvalidValue));
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
    application_user_password: String,
    application_user_reset_password_token_value: String
}

impl Incoming {
    pub fn into_inner(
        self
    ) -> (i64, String, String) {
        return (
            self.application_user_id,
            self.application_user_password,
            self.application_user_reset_password_token_value
        );
    }
}