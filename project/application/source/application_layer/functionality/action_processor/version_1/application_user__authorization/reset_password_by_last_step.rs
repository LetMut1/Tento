use crate::application_layer::data::action_processor_result::ActionProcessorResult;
use crate::application_layer::data::action_processor_result::WorkflowPrecedent;
use crate::domain_layer::data::entity::application_user_access_refresh_token::ApplicationUserAccessRefreshToken;
use crate::domain_layer::data::entity::application_user_device::ApplicationUserDevice_Id;
use crate::domain_layer::data::entity::application_user_reset_password_token::ApplicationUserResetPasswordToken_3;
use crate::domain_layer::data::entity::application_user_reset_password_token::ApplicationUserResetPasswordToken_4;
use crate::domain_layer::data::entity::application_user_reset_password_token::ApplicationUserResetPasswordToken_Value;
use crate::domain_layer::data::entity::application_user_reset_password_token::ApplicationUserResetPasswordToken_WrongEnterTriesQuantity;
use crate::domain_layer::data::entity::application_user_reset_password_token::ApplicationUserResetPasswordToken;
use crate::domain_layer::data::entity::application_user::ApplicationUser_4;
use crate::domain_layer::data::entity::application_user::ApplicationUser_Id;
use crate::domain_layer::data::entity::application_user::ApplicationUser_Password;
use crate::domain_layer::functionality::service::encoder::Encoder;
use crate::domain_layer::functionality::service::validator::Validator;
use crate::infrastructure_layer::data::argument_result::ArgumentResult;
use crate::infrastructure_layer::data::argument_result::InvalidArgument;
use crate::infrastructure_layer::data::environment_configuration::EnvironmentConfiguration;
use crate::infrastructure_layer::data::error_auditor::BacktracePart;
use crate::infrastructure_layer::data::error_auditor::BaseError;
use crate::infrastructure_layer::data::error_auditor::ErrorAuditor;
use crate::infrastructure_layer::data::error_auditor::ResourceError;
use crate::infrastructure_layer::data::error_auditor::RuntimeError;
use crate::infrastructure_layer::data::void::Void;
use crate::infrastructure_layer::functionality::repository::postgresql_repository::PostgresqlRepository;
use crate::infrastructure_layer::functionality::service::expiration_time_checker::ExpirationTimeChecker;
use crate::infrastructure_layer::functionality::service::expiration_time_checker::UnixTime;
use crate::infrastructure_layer::functionality::service::resolver::CloudMessage;
use crate::infrastructure_layer::functionality::service::resolver::Resolver;
use extern_crate::bb8_postgres::PostgresConnectionManager as PostgresqlConnectionManager;
use extern_crate::bb8_redis::RedisConnectionManager;
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
        _environment_configuration: &'a EnvironmentConfiguration,
        database_1_postgresql_connection_pool: &'a Pool<PostgresqlConnectionManager<T>>,
        database_2_postgresql_connection_pool: &'a Pool<PostgresqlConnectionManager<T>>,
        _redis_connection_pool: &'a Pool<RedisConnectionManager>,
        incoming: Incoming
    ) -> Result<ArgumentResult<ActionProcessorResult<Void>>, ErrorAuditor>
    where
        T: MakeTlsConnect<Socket> + Clone + Send + Sync + 'static,
        <T as MakeTlsConnect<Socket>>::Stream: Send + Sync,
        <T as MakeTlsConnect<Socket>>::TlsConnect: Send,
        <<T as MakeTlsConnect<Socket>>::TlsConnect as TlsConnect<Socket>>::Future: Send
    {
        let is_valid_value = match Validator::<ApplicationUserResetPasswordToken_Value>::is_valid(&incoming.application_user_reset_password_token_value) {
            Ok(is_valid_value_) => is_valid_value_,
            Err(mut error) => {
                error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                return Err(error);
            }
        };

        if !is_valid_value {
            return Ok(ArgumentResult::InvalidArgument { invalid_argument: InvalidArgument::ApplicationUserResetPasswordToken_Value });
        }

        if !Validator::<ApplicationUser_Id>::is_valid(incoming.application_user_id) {
            return Ok(ArgumentResult::InvalidArgument { invalid_argument: InvalidArgument::ApplicationUser_Id });
        }

        if !Validator::<ApplicationUser_Password>::is_valid(&incoming.application_user_password) {
            return Ok(ArgumentResult::InvalidArgument { invalid_argument: InvalidArgument::ApplicationUser_Password });
        }

        if !Validator::<ApplicationUserDevice_Id>::is_valid(&incoming.application_user_device_id) {
            return Ok(ArgumentResult::InvalidArgument { invalid_argument: InvalidArgument::ApplicationUserDevice_Id });
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

        let application_user_reset_password_token = match PostgresqlRepository::<ApplicationUserResetPasswordToken_3>::find_1(
            database_2_postgresql_connection,
            incoming.application_user_id,
            &incoming.application_user_device_id
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
                        subject: ActionProcessorResult::WorkflowPrecedent {
                            workflow_precedent: WorkflowPrecedent::ApplicationUserResetPasswordToken_NotFound
                        }
                    }
                );
            }
        };

        if ExpirationTimeChecker::<UnixTime>::is_expired(application_user_reset_password_token_.get_expires_at().get()) {
            if let Err(mut error) = PostgresqlRepository::<ApplicationUserResetPasswordToken<'_>>::delete(
                database_2_postgresql_connection,
                incoming.application_user_id,
                &incoming.application_user_device_id
            ).await {
                error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                return Err(error);
            }

            return Ok(
                ArgumentResult::Ok {
                    subject: ActionProcessorResult::WorkflowPrecedent {
                        workflow_precedent: WorkflowPrecedent::ApplicationUserResetPasswordToken_AlreadyExpired
                    }
                }
            );
        }

        if !application_user_reset_password_token_.get_is_approved().get() {
            return Ok(
                ArgumentResult::Ok {
                    subject: ActionProcessorResult::WorkflowPrecedent {
                        workflow_precedent: WorkflowPrecedent::ApplicationUserResetPasswordToken_IsNotApproved
                    }
                }
            );
        }

        if application_user_reset_password_token_.get_value().get() != incoming.application_user_reset_password_token_value.get() {
            let application_user_reset_password_token_wrong_enter_tries_quantity = match application_user_reset_password_token_.get_wrong_enter_tries_quantity().get()
                .checked_add(1) {
                Some(application_user_reset_password_token_wrong_enter_tries_quantity_) => application_user_reset_password_token_wrong_enter_tries_quantity_,
                None => {
                    return Err(
                        ErrorAuditor::new(
                            BaseError::create_out_of_range(),
                            BacktracePart::new(line!(), file!(), None)
                        )
                    );
                }
            };

            if application_user_reset_password_token_wrong_enter_tries_quantity <= ApplicationUserResetPasswordToken::WRONG_ENTER_TRIES_QUANTITY_LIMIT {
                application_user_reset_password_token_.set_wrong_enter_tries_quantity(
                    ApplicationUserResetPasswordToken_WrongEnterTriesQuantity::new(application_user_reset_password_token_wrong_enter_tries_quantity)
                );

                if let Err(mut error) = PostgresqlRepository::<ApplicationUserResetPasswordToken_4>::update(
                    database_2_postgresql_connection,
                    &application_user_reset_password_token_,
                    incoming.application_user_id,
                    &incoming.application_user_device_id
                ).await {
                    error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                    return Err(error);
                }
            } else {
                if let Err(mut error) = PostgresqlRepository::<ApplicationUserResetPasswordToken<'_>>::delete(
                    database_2_postgresql_connection,
                    incoming.application_user_id,
                    &incoming.application_user_device_id
                ).await {
                    error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                    return Err(error);
                }
            }

            return Ok(
                ArgumentResult::Ok {
                    subject: ActionProcessorResult::WorkflowPrecedent {
                        workflow_precedent: WorkflowPrecedent::ApplicationUserResetPasswordToken_WrongValue
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

        let application_user = match PostgresqlRepository::<ApplicationUser_4>::find_3(
            database_1_postgresql_connection,
            incoming.application_user_id
        ).await {
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
                        subject: ActionProcessorResult::WorkflowPrecedent {
                            workflow_precedent: WorkflowPrecedent::ApplicationUser_NotFound
                        }
                    }
                );
            }
        };

        let password_hash = match Encoder::<ApplicationUser_Password>::encode(
            &incoming.application_user_password
        ) {
            Ok(password_hash_) => password_hash_,
            Err(mut error) => {
                error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                return Err(error);
            }
        };

        application_user_.set_password_hash(password_hash);

        if let Err(mut error) = PostgresqlRepository::<ApplicationUser_4>::update(
            database_1_postgresql_connection,
            &application_user_,
            incoming.application_user_id
        ).await {
            error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

            return Err(error);
        }

        if let Err(mut error) = PostgresqlRepository::<ApplicationUserResetPasswordToken<'_>>::delete(
            database_2_postgresql_connection,
            incoming.application_user_id,
            &incoming.application_user_device_id
        ).await {
            error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

            return Err(error);
        }

        if let Err(mut error) = PostgresqlRepository::<ApplicationUserAccessRefreshToken<'_>>::delete_2(
            &*database_2_postgresql_pooled_connection,
            incoming.application_user_id,
        ).await {
            error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

            return Err(error);
        }

        Resolver::<CloudMessage>::deauthorize_application_user_from_all_devices();

        return Ok(ArgumentResult::Ok { subject: ActionProcessorResult::Void });
    }
}

#[cfg_attr(feature = "facilitate_non_automatic_functional_testing", derive(Serialize))]
#[derive(Deserialize)]
#[serde(crate = "extern_crate::serde")]
pub struct Incoming {
    application_user_device_id: ApplicationUserDevice_Id,
    application_user_id: ApplicationUser_Id,
    application_user_password: ApplicationUser_Password,
    application_user_reset_password_token_value: ApplicationUserResetPasswordToken_Value
}