use crate::application_layer::data::unified_report::UnifiedReport;
use crate::domain_layer::data::entity::application_user::ApplicationUser;
use crate::domain_layer::data::entity::application_user::ApplicationUser_Email;
use crate::domain_layer::functionality::service::validator::Validator;
use crate::infrastructure_layer::data::error_auditor::BacktracePart;
use crate::infrastructure_layer::data::error_auditor::BaseError;
use crate::infrastructure_layer::data::error_auditor::ErrorAuditor;
use crate::infrastructure_layer::data::error_auditor::ResourceError;
use crate::infrastructure_layer::data::error_auditor::RuntimeError;
use crate::infrastructure_layer::data::invalid_argument_result::InvalidArgument;
use crate::infrastructure_layer::data::invalid_argument_result::InvalidArgumentResult;
use crate::infrastructure_layer::data::pushable_environment_configuration::PushableEnvironmentConfiguration;
use crate::infrastructure_layer::data::void::Void;
use crate::infrastructure_layer::functionality::repository::postgresql_repository::PostgresqlRepository;
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
        _pushable_environment_configuration: &'a PushableEnvironmentConfiguration,
        database_1_postgresql_connection_pool: &'a Pool<PostgresqlConnectionManager<T>>,
        _database_2_postgresql_connection_pool: &'a Pool<PostgresqlConnectionManager<T>>,
        _database_1_redis_connection_pool: &'a Pool<RedisConnectionManager>,
        incoming: Incoming,
    ) -> Result<InvalidArgumentResult<UnifiedReport<Outcoming, Void>>, ErrorAuditor>
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

        let is_exist = match PostgresqlRepository::<ApplicationUser<'_>>::is_exist_2(
            &*database_1_postgresql_pooled_connection,
            &incoming.application_user_email,
        )
        .await
        {
            Ok(is_exist_) => is_exist_,
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

        let outcoming = Outcoming {
            result: is_exist,
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
}

#[cfg_attr(
    feature = "manual_testing",
    derive(Deserialize)
)]
#[derive(Serialize)]
#[serde(crate = "extern_crate::serde")]
pub struct Outcoming {
    result: bool,
}
