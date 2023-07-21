use crate::application_layer::data::common_precedent::CommonPrecedent;
use crate::application_layer::data::unified_report::UnifiedReport;
use crate::domain_layer::data::entity::application_user::ApplicationUser_Email;
use crate::domain_layer::data::entity::application_user_device::ApplicationUserDevice_Id;
use crate::domain_layer::data::entity::application_user_registration_token::ApplicationUserRegistrationToken;
use crate::domain_layer::data::entity::application_user_registration_token::ApplicationUserRegistrationToken2;
use crate::domain_layer::data::entity::application_user_registration_token::ApplicationUserRegistrationToken6;
use crate::domain_layer::data::entity::application_user_registration_token::ApplicationUserRegistrationToken_CanBeResentFrom;
use crate::domain_layer::functionality::service::email_sender::EmailSender;
use crate::domain_layer::functionality::service::generator::Generator;
use crate::domain_layer::functionality::service::validator::Validator;
use crate::infrastructure_layer::data::error_auditor::BacktracePart;
use crate::infrastructure_layer::data::error_auditor::BaseError;
use crate::infrastructure_layer::functionality::repository::postgresql_repository::update::Update8;
use crate::infrastructure_layer::data::error_auditor::ErrorAuditor;
use crate::infrastructure_layer::data::error_auditor::ResourceError;
use crate::infrastructure_layer::data::error_auditor::RuntimeError;
use crate::infrastructure_layer::functionality::repository::postgresql_repository::by::By5;
use crate::infrastructure_layer::data::invalid_argument_result::InvalidArgument;
use crate::infrastructure_layer::data::invalid_argument_result::InvalidArgumentResult;
use crate::infrastructure_layer::functionality::repository::postgresql_repository::PostgresqlRepository;
use crate::infrastructure_layer::functionality::service::expiration_time_checker::ExpirationTimeChecker;
use crate::infrastructure_layer::functionality::service::expiration_time_checker::UnixTime;
use extern_crate::bb8::Pool;
use extern_crate::bb8_postgres::PostgresConnectionManager as PostgresqlConnectionManager;
use extern_crate::bb8_redis::RedisConnectionManager;
use extern_crate::macro_rules::r#enum;
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
        _database_1_postgresql_connection_pool: &'a Pool<PostgresqlConnectionManager<T>>,
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
        let is_valid_email = match Validator::<ApplicationUser_Email>::is_valid(&incoming.application_user_email) {
            Ok(is_valid_email_) => is_valid_email_,
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

        if !is_valid_email {
            return Ok(
                InvalidArgumentResult::InvalidArgument {
                    invalid_argument: InvalidArgument::ApplicationUser_Email,
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

        let by_5 = By5 {
            application_user_email: &incoming.application_user_email,
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

        let application_user_registration_token = match PostgresqlRepository::<ApplicationUserRegistrationToken6>::find_1(
            database_2_postgresql_connection,
            &by_5,
        )
        .await
        {
            Ok(application_user_registration_token_) => application_user_registration_token_,
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

        let mut application_user_registration_token_ = match application_user_registration_token {
            Some(application_user_registration_token__) => application_user_registration_token__,
            None => {
                return Ok(
                    InvalidArgumentResult::Ok {
                        subject: UnifiedReport::precedent(Precedent::ApplicationUserRegistrationToken_NotFound),
                    },
                );
            }
        };

        if ExpirationTimeChecker::<UnixTime>::is_expired(application_user_registration_token_.expires_at.0) {
            if let Err(mut error) = PostgresqlRepository::<ApplicationUserRegistrationToken<'_>>::delete(
                database_2_postgresql_connection,
                &by_5,
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
                    subject: UnifiedReport::precedent(Precedent::ApplicationUserRegistrationToken_AlreadyExpired),
                },
            );
        }

        if application_user_registration_token_.is_approved.0 {
            return Ok(
                InvalidArgumentResult::Ok {
                    subject: UnifiedReport::precedent(Precedent::ApplicationUserRegistrationToken_AlreadyApproved),
                },
            );
        }

        if !ExpirationTimeChecker::<UnixTime>::is_expired(application_user_registration_token_.can_be_resent_from.0) {
            return Ok(
                InvalidArgumentResult::Ok {
                    subject: UnifiedReport::precedent(Precedent::ApplicationUserRegistrationToken_TimeToResendHasNotCome),
                },
            );
        }

        application_user_registration_token_.can_be_resent_from = match Generator::<ApplicationUserRegistrationToken_CanBeResentFrom>::generate() {
            Ok(application_user_registration_token_can_be_resent_from_) => application_user_registration_token_can_be_resent_from_,
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

        if let Err(mut error) = PostgresqlRepository::<ApplicationUserRegistrationToken2>::update(
            database_2_postgresql_connection,
            &Update8 {
                application_user_registration_token_can_be_resent_from: application_user_registration_token_.can_be_resent_from,
            },
            &by_5,
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

        if let Err(mut error) = EmailSender::<ApplicationUserRegistrationToken<'_>>::send(
            &application_user_registration_token_.value,
            &incoming.application_user_email,
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
            application_user_registration_token_can_be_resent_from: application_user_registration_token_.can_be_resent_from,
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
    application_user_email: ApplicationUser_Email,
    application_user_device_id: ApplicationUserDevice_Id,
}

#[cfg_attr(
    feature = "manual_testing",
    derive(Deserialize)
)]
#[derive(Serialize)]
#[serde(crate = "extern_crate::serde")]
pub struct Outcoming {
    application_user_registration_token_can_be_resent_from: ApplicationUserRegistrationToken_CanBeResentFrom,
}

r#enum!(
    pub enum Precedent {
        CommonPrecedent::ApplicationUserRegistrationToken_NotFound,
        CommonPrecedent::ApplicationUserRegistrationToken_AlreadyExpired,
        CommonPrecedent::ApplicationUserRegistrationToken_AlreadyApproved,
        CommonPrecedent::ApplicationUserRegistrationToken_TimeToResendHasNotCome,
    }
);
