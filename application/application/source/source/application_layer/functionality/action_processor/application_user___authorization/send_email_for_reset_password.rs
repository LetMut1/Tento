use crate::application_layer::data::common_precedent::CommonPrecedent;
use crate::application_layer::data::unified_report_::UnifiedReport;
use crate::domain_layer::data::entity::application_user::ApplicationUser5;
use crate::domain_layer::data::entity::application_user::ApplicationUser_Id;
use crate::domain_layer::data::entity::application_user_device::ApplicationUserDevice_Id;
use crate::domain_layer::data::entity::application_user_reset_password_token::ApplicationUserResetPasswordToken;
use crate::domain_layer::data::entity::application_user_reset_password_token::ApplicationUserResetPasswordToken2;
use crate::domain_layer::data::entity::application_user_reset_password_token::ApplicationUserResetPasswordToken6;
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
use crate::infrastructure_layer::functionality::repository::postgresql_repository::by::By3;
use crate::infrastructure_layer::functionality::repository::postgresql_repository::by::By4;
use crate::infrastructure_layer::functionality::repository::postgresql_repository::update::Update13;
use crate::infrastructure_layer::functionality::repository::postgresql_repository::PostgresqlRepository;
use crate::infrastructure_layer::functionality::service::expiration_time_checker::ExpirationTimeChecker;
use crate::infrastructure_layer::functionality::service::expiration_time_checker::UnixTime;
use bb8::Pool;
use bb8_postgres::PostgresConnectionManager as PostgresqlConnectionManager;
use bb8_redis::RedisConnectionManager;
use macro_rules::r#enum;
use std::clone::Clone;
use std::marker::Send;
use std::marker::Sync;
use tokio_postgres::tls::MakeTlsConnect;
use tokio_postgres::tls::TlsConnect;
use tokio_postgres::Socket;
pub use action_processor_incoming_outcoming::action_processor::application_user___authorization::send_email_for_reset_password::Incoming;
pub use action_processor_incoming_outcoming::action_processor::application_user___authorization::send_email_for_reset_password::Outcoming;

pub struct SendEmailForResetPassword;

impl SendEmailForResetPassword {
    pub async fn process<'a, T>(
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

        let application_user = match PostgresqlRepository::<ApplicationUser5>::find_1(
            &*database_1_postgresql_pooled_connection,
            &By3 {
                application_user_id: incoming.application_user_id,
            },
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

        let by_4 = By4 {
            application_user_id: incoming.application_user_id,
            application_user_device_id: &incoming.application_user_device_id,
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

        let application_user_reset_password_token = match PostgresqlRepository::<ApplicationUserResetPasswordToken6>::find_1(
            database_2_postgresql_connection,
            &by_4,
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

        if ExpirationTimeChecker::<UnixTime>::is_expired(application_user_reset_password_token_.expires_at.0) {
            if let Err(mut error) = PostgresqlRepository::<ApplicationUserResetPasswordToken<'_>>::delete(
                database_2_postgresql_connection,
                &by_4,
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

        if application_user_reset_password_token_.is_approved.0 {
            return Ok(
                InvalidArgumentResult::Ok {
                    subject: UnifiedReport::precedent(Precedent::ApplicationUserResetPasswordToken_AlreadyApproved),
                },
            );
        }

        if !ExpirationTimeChecker::<UnixTime>::is_expired(application_user_reset_password_token_.can_be_resent_from.0) {
            return Ok(
                InvalidArgumentResult::Ok {
                    subject: UnifiedReport::precedent(Precedent::ApplicationUserResetPasswordToken_TimeToResendHasNotCome),
                },
            );
        }

        application_user_reset_password_token_.can_be_resent_from = match Generator::<ApplicationUserResetPasswordToken_CanBeResentFrom>::generate() {
            Ok(application_user_reset_password_token_can_be_resent_from_) => application_user_reset_password_token_can_be_resent_from_,
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

        if let Err(mut error) = PostgresqlRepository::<ApplicationUserResetPasswordToken2>::update(
            database_2_postgresql_connection,
            &Update13 {
                application_user_reset_password_token_can_be_resent_from: application_user_reset_password_token_.can_be_resent_from,
            },
            &by_4,
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
            &application_user_reset_password_token_.value,
            &application_user_.email,
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
            application_user_registration_token_can_be_resent_from: application_user_reset_password_token_.can_be_resent_from,
        };

        return Ok(
            InvalidArgumentResult::Ok {
                subject: UnifiedReport::filled(outcoming),
            },
        );
    }
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
