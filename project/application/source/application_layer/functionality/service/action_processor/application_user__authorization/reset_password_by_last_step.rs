use crate::application_layer::data::action_processor_result::ActionProcessorResult;
use crate::application_layer::data::action_processor_result::UserWorkflowPrecedent;
use crate::domain_layer::data::entity::application_user_reset_password_token::ApplicationUserResetPasswordToken;
use crate::domain_layer::functionality::service::application_user__password_hash_resolver::ApplicationUser_PasswordHashResolver;
use crate::domain_layer::functionality::service::application_user__validator::ApplicationUser_Validator;
use crate::domain_layer::functionality::service::application_user_reset_password_token__expiration_time_resolver::ApplicationUserResetPasswordToken_ExpirationTimeResolver;
use crate::domain_layer::functionality::service::application_user_reset_password_token__validator::ApplicationUserResetPasswordToken_Validator;
use crate::domain_layer::functionality::service::application_user_reset_password_token__wrong_enter_tries_quantity_incrementor::ApplicationUserResetPasswordToken_WrongEnterTriesQuantityIncrementor;
use crate::infrastructure_layer::data::argument_result::ArgumentResult;
use crate::infrastructure_layer::data::argument_result::InvalidArgument;
use crate::infrastructure_layer::data::error_auditor::BacktracePart;
use crate::infrastructure_layer::data::error_auditor::BaseError;
use crate::infrastructure_layer::data::error_auditor::ErrorAuditor;
use crate::infrastructure_layer::data::error_auditor::ResourceError;
use crate::infrastructure_layer::data::error_auditor::RuntimeError;
use crate::infrastructure_layer::data::void::Void;
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
    pub async fn process<'a, T>(
        database_1_postgresql_connection_pool: &'a Pool<PostgresqlConnectionManager<T>>,
        database_2_postgresql_connection_pool: &'a Pool<PostgresqlConnectionManager<T>>,
        incoming: Incoming
    ) -> Result<ArgumentResult<ActionProcessorResult<Void>>, ErrorAuditor>
    where
        T: MakeTlsConnect<Socket> + Clone + Send + Sync + 'static,
        <T as MakeTlsConnect<Socket>>::Stream: Send + Sync,
        <T as MakeTlsConnect<Socket>>::TlsConnect: Send,
        <<T as MakeTlsConnect<Socket>>::TlsConnect as TlsConnect<Socket>>::Future: Send
    {
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
            return Ok(ArgumentResult::InvalidArgument { invalid_argument: InvalidArgument::ApplicationUserResetPasswordToken_Value });
        }

        if ApplicationUser_Validator::is_valid_password(incoming.application_user_password.as_str()) {
            return Ok(ArgumentResult::InvalidArgument { invalid_argument: InvalidArgument::ApplicationUser_Password });
        }

        let database_2_postgresql_pooled_connection = match database_2_postgresql_connection_pool.get().await {
            Ok(database_2_postgresql_pooled_connection_) => database_2_postgresql_pooled_connection_,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RuntimeError { runtime_error: RuntimeError::ResourceError { resource_error: ResourceError::ConnectionPoolPostgresqlError { bb8_postgresql_error: error } } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        };
        let database_2_postgresql_connection = &*database_2_postgresql_pooled_connection;

        let application_user_reset_password_token = match ApplicationUserResetPasswordToken_PostgresqlRepository::find_1(
            database_2_postgresql_connection, incoming.application_user_id
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
                return Ok(
                    ArgumentResult::Ok {
                        subject: ActionProcessorResult::UserWorkflowPrecedent {
                            user_workflow_precedent: UserWorkflowPrecedent::ApplicationUserResetPasswordToken_NotFound
                        }
                    }
                );
            }
        };

        if ApplicationUserResetPasswordToken_ExpirationTimeResolver::is_expired(&application_user_reset_password_token_) {
            if let Err(mut error) = ApplicationUserResetPasswordToken_PostgresqlRepository::delete(
                database_2_postgresql_connection, application_user_reset_password_token_.get_application_user_id()
            ).await {
                error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                return Err(error);
            }

            return Ok(
                ArgumentResult::Ok {
                    subject: ActionProcessorResult::UserWorkflowPrecedent {
                        user_workflow_precedent: UserWorkflowPrecedent::ApplicationUserResetPasswordToken_AlreadyExpired
                    }
                }
            );
        }

        if !application_user_reset_password_token_.get_is_approved() {
            return Ok(
                ArgumentResult::Ok {
                    subject: ActionProcessorResult::UserWorkflowPrecedent {
                        user_workflow_precedent: UserWorkflowPrecedent::ApplicationUserResetPasswordToken_IsNotApproved
                    }
                }
            );
        }

        if application_user_reset_password_token_.get_value() != incoming.application_user_reset_password_token_value.as_str() {
            if let Err(mut error) = ApplicationUserResetPasswordToken_WrongEnterTriesQuantityIncrementor::increment(&mut application_user_reset_password_token_) {
                error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                return Err(error);
            }

            if application_user_reset_password_token_.get_wrong_enter_tries_quantity() <= ApplicationUserResetPasswordToken::WRONG_ENTER_TRIES_QUANTITY_LIMIT {
                if let Err(mut error) = ApplicationUserResetPasswordToken_PostgresqlRepository::update(
                    database_2_postgresql_connection,
                    &mut application_user_reset_password_token_,
                    Update { application_user_reset_password_token_expires_at: false }
                ).await {
                    error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                    return Err(error);
                }
            } else {
                if let Err(mut error) = ApplicationUserResetPasswordToken_PostgresqlRepository::delete(
                    database_2_postgresql_connection, application_user_reset_password_token_.get_application_user_id()
                ).await {
                    error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                    return Err(error);
                }
            }

            return Ok(
                ArgumentResult::Ok {
                    subject: ActionProcessorResult::UserWorkflowPrecedent {
                        user_workflow_precedent: UserWorkflowPrecedent::ApplicationUserResetPasswordToken_WrongValue
                    }
                }
            );
        }

        let database_1_postgresql_pooled_connection = match database_1_postgresql_connection_pool.get().await {
            Ok(database_1_postgresql_pooled_connection_) => database_1_postgresql_pooled_connection_,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RuntimeError { runtime_error: RuntimeError::ResourceError { resource_error: ResourceError::ConnectionPoolPostgresqlError { bb8_postgresql_error: error } } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        };
        let database_1_postgresql_connection = &*database_1_postgresql_pooled_connection;

        let application_user = match ApplicationUser_PostgresqlRepository::find_3(database_1_postgresql_connection, incoming.application_user_id).await {
            Ok(application_user_) => application_user_,
            Err(mut error) => {
                error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                return Err(error);
            }
        };
        let mut application_user_ = match application_user {
            Some(application_user__) => application_user__,
            None => {
                return Ok(
                    ArgumentResult::Ok {
                        subject: ActionProcessorResult::UserWorkflowPrecedent {
                            user_workflow_precedent: UserWorkflowPrecedent::ApplicationUser_NotFound
                        }
                    }
                );
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

        if let Err(mut error) = ApplicationUser_PostgresqlRepository::update(database_1_postgresql_connection, &application_user_).await {
            error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

            return Err(error);
        }

        if let Err(mut error) = ApplicationUserResetPasswordToken_PostgresqlRepository::delete(
            database_2_postgresql_connection, application_user_reset_password_token_.get_application_user_id()
        ).await {
            error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

            return Err(error);
        }

        if let Err(mut error) = ApplicationUserAccessRefreshToken_PostgresqlRepository::delete_2(
            &*database_2_postgresql_pooled_connection, application_user_.get_id()
        ).await {
            error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

            return Err(error);
        }

        CloudMessageResolver::deauthorize_application_user_from_all_devices();

        return Ok(ArgumentResult::Ok { subject: ActionProcessorResult::Void });
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