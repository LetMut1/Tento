use crate::application_layer::data::common_precedent::CommonPrecedent;
use crate::application_layer::data::unified_report::UnifiedReport;
use crate::domain_layer::data::entity::application_user::ApplicationUser_5;
use crate::domain_layer::data::entity::application_user::ApplicationUser_Id;
use crate::domain_layer::data::entity::application_user_device::ApplicationUserDevice_Id;
use crate::domain_layer::data::entity::application_user_reset_password_token::ApplicationUserResetPasswordToken;
use crate::domain_layer::data::entity::application_user_reset_password_token::ApplicationUserResetPasswordToken_2;
use crate::domain_layer::data::entity::application_user_reset_password_token::ApplicationUserResetPasswordToken_6;
use crate::domain_layer::data::entity::application_user_reset_password_token::ApplicationUserResetPasswordToken_CanBeResentFrom;
use crate::domain_layer::functionality::service::email_sender::EmailSender;
use crate::domain_layer::functionality::service::generator::Generator;
use crate::domain_layer::functionality::service::validator::Validator;
use crate::infrastructure_layer::data::error_auditor::BacktracePart;
use crate::infrastructure_layer::data::error_auditor::BaseError;
use crate::infrastructure_layer::data::error_auditor::ErrorAuditor;
use crate::infrastructure_layer::data::error_auditor::ResourceError;
use crate::infrastructure_layer::data::error_auditor::RuntimeError;
use crate::infrastructure_layer::data::invalid_argument_result::InvalidArgument;
use crate::infrastructure_layer::data::invalid_argument_result::InvalidArgumentResult;
use crate::infrastructure_layer::data::pushable_environment_configuration::PushableEnvironmentConfiguration;
use crate::infrastructure_layer::functionality::repository::postgresql_repository::PostgresqlRepository;
use crate::infrastructure_layer::functionality::service::expiration_time_checker::ExpirationTimeChecker;
use crate::infrastructure_layer::functionality::service::expiration_time_checker::UnixTime;
use crate::infrastructure_layer::functionality::service::macro_rules::r#enum;
use extern_crate::bb8::Pool;
use extern_crate::bb8_postgres::PostgresConnectionManager as PostgresqlConnectionManager;
use extern_crate::bb8_redis::RedisConnectionManager;
use extern_crate::serde::Deserialize;
use extern_crate::serde::Serialize;
use extern_crate::tokio_postgres::tls::MakeTlsConnect;
use extern_crate::tokio_postgres::tls::TlsConnect;
use extern_crate::tokio_postgres::Socket;
use std::clone::Clone;
use std::marker::Send;
use std::marker::Sync;

pub struct ActionProcessor;

impl ActionProcessor {
    pub async fn process<'a, T>(
        pushable_environment_configuration: &'a PushableEnvironmentConfiguration,
        database_1_postgresql_connection_pool: &'a Pool<PostgresqlConnectionManager<T>>,
        database_2_postgresql_connection_pool: &'a Pool<PostgresqlConnectionManager<T>>,
        _database_1_redis_connection_pool: &'a Pool<RedisConnectionManager>,
        incoming: Incoming,
    ) -> Result<InvalidArgumentResult<UnifiedReport<Outcoming, Precedent>>, ErrorAuditor>
    where
        T: MakeTlsConnect<Socket> + Clone + Send + Sync + 'static,
        <T as MakeTlsConnect<Socket>>::Stream: Send + Sync,
        <T as MakeTlsConnect<Socket>>::TlsConnect: Send,
        <<T as MakeTlsConnect<Socket>>::TlsConnect as TlsConnect<Socket>>::Future: Send,
    {
        if !Validator::<ApplicationUser_Id>::is_valid(incoming.application_user_id) {
            return Ok(
                InvalidArgumentResult::InvalidArgument {
                    invalid_argument: InvalidArgument::ApplicationUser_Id,
                },
            );
        }

        if !Validator::<ApplicationUserDevice_Id>::is_valid(&incoming.application_user_device_id) {
            return Ok(
                InvalidArgumentResult::InvalidArgument {
                    invalid_argument: InvalidArgument::ApplicationUserDevice_Id,
                },
            );
        }

        let database_1_postgresql_pooled_connection = match database_1_postgresql_connection_pool.get().await {
            Ok(database_1_postgresql_pooled_connection_) => database_1_postgresql_pooled_connection_,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RuntimeError {
                            runtime_error: RuntimeError::ResourceError {
                                resource_error: ResourceError::ConnectionPoolPostgresqlError {
                                    bb8_postgresql_error: error,
                                },
                            },
                        },
                        BacktracePart::new(
                            line!(),
                            file!(),
                            None,
                        ),
                    ),
                );
            }
        };

        let application_user = match PostgresqlRepository::<ApplicationUser_5>::find_3(
            &*database_1_postgresql_pooled_connection,
            incoming.application_user_id,
        )
        .await
        {
            Ok(application_user_) => application_user_,
            Err(mut error) => {
                error.add_backtrace_part(
                    BacktracePart::new(
                        line!(),
                        file!(),
                        None,
                    ),
                );

                return Err(error);
            }
        };

        let application_user_ = match application_user {
            Some(application_user__) => application_user__,
            None => {
                return Ok(
                    InvalidArgumentResult::Ok {
                        subject: UnifiedReport::precedent(Precedent::ApplicationUser_NotFound),
                    },
                );
            }
        };

        let database_2_postgresql_pooled_connection = match database_2_postgresql_connection_pool.get().await {
            Ok(database_2_postgresql_pooled_connection_) => database_2_postgresql_pooled_connection_,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RuntimeError {
                            runtime_error: RuntimeError::ResourceError {
                                resource_error: ResourceError::ConnectionPoolPostgresqlError {
                                    bb8_postgresql_error: error,
                                },
                            },
                        },
                        BacktracePart::new(
                            line!(),
                            file!(),
                            None,
                        ),
                    ),
                );
            }
        };

        let database_2_postgresql_connection = &*database_2_postgresql_pooled_connection;

        let application_user_reset_password_token =
            match PostgresqlRepository::<ApplicationUserResetPasswordToken_6>::find_1(
                database_2_postgresql_connection,
                incoming.application_user_id,
                &incoming.application_user_device_id,
            )
            .await
            {
                Ok(application_user_reset_password_token_) => application_user_reset_password_token_,
                Err(mut error) => {
                    error.add_backtrace_part(
                        BacktracePart::new(
                            line!(),
                            file!(),
                            None,
                        ),
                    );

                    return Err(error);
                }
            };

        let mut application_user_reset_password_token_ = match application_user_reset_password_token {
            Some(application_user_reset_password_token__) => application_user_reset_password_token__,
            None => {
                return Ok(
                    InvalidArgumentResult::Ok {
                        subject: UnifiedReport::precedent(Precedent::ApplicationUserResetPasswordToken_NotFound),
                    },
                );
            }
        };

        if ExpirationTimeChecker::<UnixTime>::is_expired(
            application_user_reset_password_token_
                .get_expires_at()
                .get(),
        ) {
            if let Err(mut error) = PostgresqlRepository::<ApplicationUserResetPasswordToken<'_>>::delete(
                database_2_postgresql_connection,
                incoming.application_user_id,
                &incoming.application_user_device_id,
            )
            .await
            {
                error.add_backtrace_part(
                    BacktracePart::new(
                        line!(),
                        file!(),
                        None,
                    ),
                );

                return Err(error);
            }

            return Ok(
                InvalidArgumentResult::Ok {
                    subject: UnifiedReport::precedent(Precedent::ApplicationUserResetPasswordToken_AlreadyExpired),
                },
            );
        }

        if application_user_reset_password_token_
            .get_is_approved()
            .get()
        {
            return Ok(
                InvalidArgumentResult::Ok {
                    subject: UnifiedReport::precedent(Precedent::ApplicationUserResetPasswordToken_AlreadyApproved),
                },
            );
        }

        if !ExpirationTimeChecker::<UnixTime>::is_expired(
            application_user_reset_password_token_
                .get_can_be_resent_from()
                .get(),
        ) {
            return Ok(
                InvalidArgumentResult::Ok {
                    subject: UnifiedReport::precedent(
                        Precedent::ApplicationUserResetPasswordToken_TimeToResendHasNotCome,
                    ),
                },
            );
        }

        let application_user_reset_password_token_can_be_resent_from =
            match Generator::<ApplicationUserResetPasswordToken_CanBeResentFrom>::generate() {
                Ok(application_user_reset_password_token_can_be_resent_from_) => {
                    application_user_reset_password_token_can_be_resent_from_
                }
                Err(mut error) => {
                    error.add_backtrace_part(
                        BacktracePart::new(
                            line!(),
                            file!(),
                            None,
                        ),
                    );

                    return Err(error);
                }
            };

        application_user_reset_password_token_
            .set_can_be_resent_from(application_user_reset_password_token_can_be_resent_from);

        if let Err(mut error) = PostgresqlRepository::<ApplicationUserResetPasswordToken_2>::update(
            database_2_postgresql_connection,
            &application_user_reset_password_token_,
            incoming.application_user_id,
            &incoming.application_user_device_id,
        )
        .await
        {
            error.add_backtrace_part(
                BacktracePart::new(
                    line!(),
                    file!(),
                    None,
                ),
            );

            return Err(error);
        }

        if let Err(mut error) = EmailSender::<ApplicationUserResetPasswordToken<'_>>::send(
            pushable_environment_configuration,
            application_user_reset_password_token_.get_value(),
            application_user_.get_email(),
            &incoming.application_user_device_id,
        ) {
            error.add_backtrace_part(
                BacktracePart::new(
                    line!(),
                    file!(),
                    None,
                ),
            );

            return Err(error);
        }

        let outcoming = Outcoming {
            application_user_registration_token_can_be_resent_from: application_user_reset_password_token_
                .get_can_be_resent_from(),
        };

        return Ok(
            InvalidArgumentResult::Ok {
                subject: UnifiedReport::filled(outcoming),
            },
        );
    }
}

#[cfg_attr(
    feature = "manual_testing",
    derive(Serialize)
)]
#[derive(Deserialize)]
#[serde(crate = "extern_crate::serde")]
pub struct Incoming {
    application_user_id: ApplicationUser_Id,
    application_user_device_id: ApplicationUserDevice_Id,
}

#[cfg_attr(
    feature = "manual_testing",
    derive(Deserialize)
)]
#[derive(Serialize)]
#[serde(crate = "extern_crate::serde")]
pub struct Outcoming {
    application_user_registration_token_can_be_resent_from: ApplicationUserResetPasswordToken_CanBeResentFrom,
}

r#enum!(
    pub enum Precedent {
        CommonPrecedent::ApplicationUser_NotFound,
        CommonPrecedent::ApplicationUserResetPasswordToken_NotFound,
        CommonPrecedent::ApplicationUserResetPasswordToken_AlreadyExpired,
        CommonPrecedent::ApplicationUserResetPasswordToken_AlreadyApproved,
        CommonPrecedent::ApplicationUserResetPasswordToken_TimeToResendHasNotCome,
    }
);
