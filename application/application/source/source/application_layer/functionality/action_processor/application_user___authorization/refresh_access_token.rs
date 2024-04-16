use crate::application_layer::data::unified_report::UnifiedReport;
use crate::domain_layer::data::entity::application_user_access_refresh_token::ApplicationUserAccessRefreshToken;
use crate::domain_layer::data::entity::application_user_access_refresh_token::ApplicationUserAccessRefreshToken1;
use crate::domain_layer::data::entity::application_user_access_refresh_token::ApplicationUserAccessRefreshToken_ExpiresAt;
use crate::domain_layer::data::entity::application_user_access_refresh_token::ApplicationUserAccessRefreshToken_ObfuscationValue;
use crate::domain_layer::data::entity::application_user_access_refresh_token::ApplicationUserAccessRefreshToken_UpdatedAt;
use crate::domain_layer::data::entity::application_user_access_token::ApplicationUserAccessToken;
use crate::domain_layer::data::entity::application_user_access_token::ApplicationUserAccessToken_ExpiresAt;
use crate::domain_layer::data::entity::application_user_access_token::ApplicationUserAccessToken_Id;
use crate::domain_layer::functionality::service::form_resolver::FormResolver;
use crate::domain_layer::functionality::service::generator::Generator;
use crate::infrastructure_layer::data::error::BacktracePart;
use crate::infrastructure_layer::data::error::Error;
use crate::infrastructure_layer::data::error::Auditor;
use crate::infrastructure_layer::data::error::Other;
use crate::infrastructure_layer::data::error::Runtime;
use crate::infrastructure_layer::data::invalid_argument_result::InvalidArgument;
use crate::infrastructure_layer::data::invalid_argument_result::InvalidArgumentResult;
use crate::infrastructure_layer::functionality::repository::postgresql::by::By4;
use crate::infrastructure_layer::functionality::repository::postgresql::update::Update2;
use crate::infrastructure_layer::functionality::repository::postgresql::PostgresqlRepository;
use crate::infrastructure_layer::functionality::service::expiration_time_checker::ExpirationTimeChecker;
use crate::infrastructure_layer::functionality::service::expiration_time_checker::unix_time::UnixTime;
use bb8::Pool;
use bb8_postgres::PostgresConnectionManager as PostgresqlConnectionManager;
use bb8_redis::RedisConnectionManager;
use std::borrow::Cow;
use std::clone::Clone;
use std::marker::Send;
use std::marker::Sync;
use tokio_postgres::tls::MakeTlsConnect;
use tokio_postgres::tls::TlsConnect;
use tokio_postgres::Socket;
use crate::application_layer::functionality::action_processor::ActionProcessor;

pub use action_processor_incoming_outcoming::action_processor::application_user___authorization::refresh_access_token::Incoming;
pub use action_processor_incoming_outcoming::action_processor::application_user___authorization::refresh_access_token::Outcoming;
pub use action_processor_incoming_outcoming::action_processor::application_user___authorization::refresh_access_token::Precedent;
pub use crate::infrastructure_layer::data::control_type::ApplicationUser__Authorization___RefreshAccessToken;

impl ActionProcessor<ApplicationUser__Authorization___RefreshAccessToken> {
    pub async fn process<'a, T>(
        _database_1_postgresql_connection_pool: &'a Pool<PostgresqlConnectionManager<T>>,
        database_2_postgresql_connection_pool: &'a Pool<PostgresqlConnectionManager<T>>,
        _database_1_redis_connection_pool: &'a Pool<RedisConnectionManager>,
        incoming: Option<Incoming>,
    ) -> Result<InvalidArgumentResult<UnifiedReport<Outcoming, Precedent>>, Auditor<Error>>
    where
        T: MakeTlsConnect<Socket> + Clone + Send + Sync + 'static,
        <T as MakeTlsConnect<Socket>>::Stream: Send + Sync,
        <T as MakeTlsConnect<Socket>>::TlsConnect: Send,
        <<T as MakeTlsConnect<Socket>>::TlsConnect as TlsConnect<Socket>>::Future: Send,
    {
        let incoming_ = match incoming {
            Some(incoming__) => incoming__,
            None => {
                return Err(
                    Auditor::<Error>::new(
                        Error::create_incoming_invalid_state(),
                        BacktracePart::new(
                            line!(),
                            file!(),
                        ),
                    ),
                );
            }
        };

        let application_user_access_token = match FormResolver::<ApplicationUserAccessToken<'_>>::from_encrypted(&incoming_.application_user_access_token_encrypted) {
            Ok(application_user_access_token_) => application_user_access_token_,
            Err(mut error) => {
                error.add_backtrace_part(
                    BacktracePart::new(
                        line!(),
                        file!(),
                    ),
                );

                return Err(error);
            }
        };

        let application_user_access_token_ = match application_user_access_token {
            InvalidArgumentResult::Ok {
                subject: application_user_access_token__,
            } => application_user_access_token__,
            InvalidArgumentResult::InvalidArgument {
                invalid_argument,
            } => {
                return Ok(
                    InvalidArgumentResult::InvalidArgument {
                        invalid_argument,
                    },
                );
            }
        };

        let by_4 = By4 {
            application_user_id: application_user_access_token_.application_user_id,
            application_user_device_id: application_user_access_token_.application_user_device_id.as_ref(),
        };

        let database_2_postgresql_pooled_connection = match database_2_postgresql_connection_pool.get().await {
            Ok(database_2_postgresql_pooled_connection_) => database_2_postgresql_pooled_connection_,
            Err(error) => {
                return Err(
                    Auditor::<Error>::new(
                        Error::Runtime {
                            runtime: Runtime::Other {
                                other: Other::new(error),
                            },
                        },
                        BacktracePart::new(
                            line!(),
                            file!(),
                        ),
                    ),
                );
            }
        };

        let database_2_postgresql_connection = &*database_2_postgresql_pooled_connection;

        let application_user_access_refresh_token = match PostgresqlRepository::<ApplicationUserAccessRefreshToken<'_>>::find_1(
            database_2_postgresql_connection,
            &by_4,
        )
        .await
        {
            Ok(application_user_access_refresh_token_) => application_user_access_refresh_token_,
            Err(mut error) => {
                error.add_backtrace_part(
                    BacktracePart::new(
                        line!(),
                        file!(),
                    ),
                );

                return Err(error);
            }
        };

        let mut application_user_access_refresh_token_ = match application_user_access_refresh_token {
            Some(application_user_access_refresh_token__) => application_user_access_refresh_token__,
            None => {
                return Ok(
                    InvalidArgumentResult::Ok {
                        subject: UnifiedReport::precedent(Precedent::ApplicationUserAccessRefreshToken_NotFound),
                    },
                );
            }
        };

        let is_valid = match FormResolver::<ApplicationUserAccessRefreshToken<'_>>::is_valid(
            &application_user_access_refresh_token_,
            &incoming_.application_user_access_refresh_token_encrypted,
        ) {
            Ok(is_valid_) => is_valid_,
            Err(mut error) => {
                error.add_backtrace_part(
                    BacktracePart::new(
                        line!(),
                        file!(),
                    ),
                );

                return Err(error);
            }
        };

        if !is_valid || application_user_access_token_.id.0 != application_user_access_refresh_token_.application_user_access_token_id.as_ref().0 {
            return Ok(
                InvalidArgumentResult::InvalidArgument {
                    invalid_argument: InvalidArgument::ApplicationUserAccessRefreshTokenEncrypted,
                },
            );
        }

        if ExpirationTimeChecker::<UnixTime>::is_expired(application_user_access_refresh_token_.expires_at.0) {
            if let Err(mut error) = PostgresqlRepository::<ApplicationUserAccessRefreshToken<'_>>::delete_1(
                database_2_postgresql_connection,
                &by_4,
            )
            .await
            {
                error.add_backtrace_part(
                    BacktracePart::new(
                        line!(),
                        file!(),
                    ),
                );

                return Err(error);
            }

            return Ok(
                InvalidArgumentResult::Ok {
                    subject: UnifiedReport::precedent(Precedent::ApplicationUserAccessRefreshToken_AlreadyExpired),
                },
            );
        }

        let expires_at = match Generator::<ApplicationUserAccessToken_ExpiresAt>::generate() {
            Ok(expires_at_) => expires_at_,
            Err(mut error) => {
                error.add_backtrace_part(
                    BacktracePart::new(
                        line!(),
                        file!(),
                    ),
                );

                return Err(error);
            }
        };

        let application_user_access_token_new = ApplicationUserAccessToken {
            id: Generator::<ApplicationUserAccessToken_Id>::generate(),
            application_user_id: application_user_access_token_.application_user_id,
            application_user_device_id: Cow::Borrowed(application_user_access_token_.application_user_device_id.as_ref()),
            expires_at,
        };

        application_user_access_refresh_token_.application_user_access_token_id = Cow::Borrowed(&application_user_access_token_new.id);

        application_user_access_refresh_token_.obfuscation_value = Generator::<ApplicationUserAccessRefreshToken_ObfuscationValue>::generate();

        application_user_access_refresh_token_.expires_at = match Generator::<ApplicationUserAccessRefreshToken_ExpiresAt>::generate() {
            Ok(application_user_access_refresh_token_expires_at) => application_user_access_refresh_token_expires_at,
            Err(mut error) => {
                error.add_backtrace_part(
                    BacktracePart::new(
                        line!(),
                        file!(),
                    ),
                );

                return Err(error);
            }
        };

        application_user_access_refresh_token_.updated_at = Generator::<ApplicationUserAccessRefreshToken_UpdatedAt>::generate();

        if let Err(mut error) = PostgresqlRepository::<ApplicationUserAccessRefreshToken1>::update(
            database_2_postgresql_connection,
            &Update2 {
                application_user_access_token_id: application_user_access_refresh_token_.application_user_access_token_id.as_ref(),
                application_user_access_refresh_token_obfuscation_value: &application_user_access_refresh_token_.obfuscation_value,
                application_user_access_refresh_token_expires_at: application_user_access_refresh_token_.expires_at,
                application_user_access_refresh_token_updated_at: application_user_access_refresh_token_.updated_at,
            },
            &by_4,
        )
        .await
        {
            error.add_backtrace_part(
                BacktracePart::new(
                    line!(),
                    file!(),
                ),
            );

            return Err(error);
        }

        let application_user_access_token_encrypted_new = match FormResolver::<ApplicationUserAccessToken<'_>>::to_encrypted(&application_user_access_token_new) {
            Ok(application_user_access_token_encrypted_new_) => application_user_access_token_encrypted_new_,
            Err(mut error) => {
                error.add_backtrace_part(
                    BacktracePart::new(
                        line!(),
                        file!(),
                    ),
                );

                return Err(error);
            }
        };

        let application_user_access_refresh_token_encrypted_new = match FormResolver::<ApplicationUserAccessRefreshToken<'_>>::to_encrypted(&application_user_access_refresh_token_) {
            Ok(application_user_access_refresh_token_encrypted_new_) => application_user_access_refresh_token_encrypted_new_,
            Err(mut error) => {
                error.add_backtrace_part(
                    BacktracePart::new(
                        line!(),
                        file!(),
                    ),
                );

                return Err(error);
            }
        };

        let outcoming = Outcoming {
            application_user_access_token_encrypted: application_user_access_token_encrypted_new,
            application_user_access_refresh_token_encrypted: application_user_access_refresh_token_encrypted_new,
        };

        return Ok(
            InvalidArgumentResult::Ok {
                subject: UnifiedReport::target_filled(outcoming),
            },
        );
    }
}