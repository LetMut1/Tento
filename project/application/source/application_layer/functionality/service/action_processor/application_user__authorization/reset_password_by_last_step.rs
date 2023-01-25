use crate::application_layer::data::action_processor_result::ActionProcessorResult;
use crate::application_layer::data::entity_workflow_exception::ApplicationUserResetPasswordToken_WorkflowException;
use crate::application_layer::data::entity_workflow_exception::ApplicationUser_WorkflowException;
use crate::domain_layer::data::entity::application_user_reset_password_token::ApplicationUserResetPasswordToken;
use crate::domain_layer::functionality::service::application_user__password_hash_resolver::ApplicationUser_PasswordHashResolver;
use crate::domain_layer::functionality::service::application_user__validator::ApplicationUser_Validator;
use crate::domain_layer::functionality::service::application_user_reset_password_token__expiration_time_resolver::ApplicationUserResetPasswordToken_ExpirationTimeResolver;
use crate::domain_layer::functionality::service::application_user_reset_password_token__validator::ApplicationUserResetPasswordToken_Validator;
use crate::domain_layer::functionality::service::application_user_reset_password_token__wrong_enter_tries_quantity_incrementor::ApplicationUserResetPasswordToken_WrongEnterTriesQuantityIncrementor;
use crate::infrastructure_layer::data::error_auditor::BacktracePart;
use crate::infrastructure_layer::data::error_auditor::BaseError;
use crate::infrastructure_layer::data::error_auditor::ErrorAuditor;
use crate::infrastructure_layer::data::error_auditor::ResourceError;
use crate::infrastructure_layer::data::error_auditor::RunTimeError;
use crate::infrastructure_layer::functionality::repository::application_user__postgresql_repository::ApplicationUser_PostgresqlRepository;
use crate::infrastructure_layer::functionality::repository::application_user_access_refresh_token__postgresql_repository::ApplicationUserAccessRefreshToken_PostgresqlRepository;
use crate::infrastructure_layer::functionality::repository::application_user_reset_password_token__postgresql_repository::ApplicationUserResetPasswordToken_PostgresqlRepository;
use crate::infrastructure_layer::functionality::repository::application_user_reset_password_token__postgresql_repository::Update;
use crate::infrastructure_layer::functionality::service::cloud_message_resolver::CloudMessageResolver;
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
    pub async fn process<T>(
        core_postgresql_connection_pool: Pool<PostgresqlConnectionManager<T>>,
        authorization_postgresql_connection_pool: Pool<PostgresqlConnectionManager<T>>,
        incoming: Incoming
    ) -> Result<ActionProcessorResult<()>, ErrorAuditor>
    where
        T: MakeTlsConnect<Socket> + Clone + Send + Sync + 'static,
        <T as MakeTlsConnect<Socket>>::Stream: Send + Sync,
        <T as MakeTlsConnect<Socket>>::TlsConnect: Send,
        <<T as MakeTlsConnect<Socket>>::TlsConnect as TlsConnect<Socket>>::Future: Send
    {                                   // TODO  !!!!! Это ресет для пользователя, забывшего пароль. НО также нужно сделать АККУРАТНО ресетпассворд для залогиневшегося пользователя с повторной отправкой старого пароля !!!!!!!!!
        let is_valid_value = match ApplicationUserResetPasswordToken_Validator::is_valid_value(
            incoming.application_user_reset_password_token_value.as_str()
        ) {
            Ok(is_valid_value_) => is_valid_value_,
            Err(mut error) => {
                error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                return Err(error);
            }
        };
        if !is_valid_value {
            return Ok(ActionProcessorResult::application_user_reset_password_token__workflow_exception(ApplicationUserResetPasswordToken_WorkflowException::InvalidValue));
        }
        if ApplicationUser_Validator::is_valid_password(incoming.application_user_password.as_str()) {
            return Ok(ActionProcessorResult::application_user__workflow_exception(ApplicationUser_WorkflowException::InvalidPassword));
        }

        let authorization_postgresql_pooled_connection = match authorization_postgresql_connection_pool.get().await {
            Ok(authorization_postgresql_pooled_connection_) => authorization_postgresql_pooled_connection_,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RunTimeError { run_time_error: RunTimeError::ResourceError { resource_error: ResourceError::ConnectionPoolPostgresqlError { bb8_postgresql_error: error } } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        };
        let authorization_postgresql_connection = &*authorization_postgresql_pooled_connection;

        let application_user_reset_password_token = match ApplicationUserResetPasswordToken_PostgresqlRepository::find_1(
            authorization_postgresql_connection, incoming.application_user_id
        ).await {
            Ok(application_user_reset_password_token_) => application_user_reset_password_token_,
            Err(mut error) => {
                error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                return Err(error);
            }
        };
        let mut application_user_reset_password_token_ = match application_user_reset_password_token {
            Some(application_user_reset_password_token__) => application_user_reset_password_token__,
            None => {
                return Ok(ActionProcessorResult::application_user_reset_password_token__workflow_exception(ApplicationUserResetPasswordToken_WorkflowException::NotFound));
            }
        };

        if !ApplicationUserResetPasswordToken_ExpirationTimeResolver::is_expired(&application_user_reset_password_token_) {
            return Ok(ActionProcessorResult::application_user_reset_password_token__workflow_exception(ApplicationUserResetPasswordToken_WorkflowException::AlreadyExpired));
        }

        if !application_user_reset_password_token_.get_is_approved() {
            return Ok(ActionProcessorResult::application_user_reset_password_token__workflow_exception(ApplicationUserResetPasswordToken_WorkflowException::IsNotApproved));
        }

        if application_user_reset_password_token_.get_value() != incoming.application_user_reset_password_token_value.as_str() {
            if let Err(mut error) = ApplicationUserResetPasswordToken_WrongEnterTriesQuantityIncrementor::increment(&mut application_user_reset_password_token_) {
                error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                return Err(error);
            }

            if application_user_reset_password_token_.get_wrong_enter_tries_quantity() <= ApplicationUserResetPasswordToken::WRONG_ENTER_TRIES_QUANTITY_LIMIT {
                if let Err(mut error) = ApplicationUserResetPasswordToken_PostgresqlRepository::update(
                    authorization_postgresql_connection,
                    &mut application_user_reset_password_token_,
                    Update { application_user_reset_password_token_expires_at: false }
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

            return Ok(ActionProcessorResult::application_user_reset_password_token__workflow_exception(ApplicationUserResetPasswordToken_WorkflowException::WrongValue));
        }

        let core_postgresql_pooled_connection = match core_postgresql_connection_pool.get().await {
            Ok(core_postgresql_pooled_connection_) => core_postgresql_pooled_connection_,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RunTimeError { run_time_error: RunTimeError::ResourceError { resource_error: ResourceError::ConnectionPoolPostgresqlError { bb8_postgresql_error: error } } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        };
        let core_postgresql_connection = &*core_postgresql_pooled_connection;

        let application_user = match ApplicationUser_PostgresqlRepository::find_3(core_postgresql_connection, incoming.application_user_id).await {
            Ok(application_user_) => application_user_,
            Err(mut error) => {
                error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                return Err(error);
            }
        };
        let mut application_user_ = match application_user {
            Some(application_user__) => application_user__,
            None => {
                return Ok(ActionProcessorResult::application_user__workflow_exception(ApplicationUser_WorkflowException::NotFound));
            }
        };

        let password_hash = match ApplicationUser_PasswordHashResolver::create(incoming.application_user_password.as_str()) {
            Ok(password_hash_) => password_hash_,
            Err(mut error) => {
                error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                return Err(error);
            }
        };

        application_user_.set_password_hash(password_hash);

        if let Err(mut error) = ApplicationUser_PostgresqlRepository::update(core_postgresql_connection, &application_user_).await {
            error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

            return Err(error);
        }

        if let Err(mut error) = ApplicationUserResetPasswordToken_PostgresqlRepository::delete(
            authorization_postgresql_connection, application_user_reset_password_token_.get_application_user_id()
        ).await {
            error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

            return Err(error);
        }

        if let Err(mut error) = ApplicationUserAccessRefreshToken_PostgresqlRepository::delete_2(
            &*authorization_postgresql_pooled_connection, application_user_.get_id()
        ).await {
            error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

            return Err(error);
        }

        CloudMessageResolver::deauthorize_application_user_from_all_devices();

        return Ok(ActionProcessorResult::outcoming(()));
    }
}

#[cfg_attr(feature = "facilitate_non_automatic_functional_testing", derive(Serialize))]
#[derive(Deserialize)]
#[serde(crate = "extern_crate::serde")]
pub struct Incoming {
    application_user_id: i64,
    application_user_password: String,
    application_user_reset_password_token_value: String
}