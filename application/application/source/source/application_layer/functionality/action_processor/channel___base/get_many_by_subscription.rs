use crate::application_layer::data::common_precedent::CommonPrecedent;
use crate::application_layer::data::unified_report_::UnifiedReport;
use crate::domain_layer::data::entity::application_user_access_token::ApplicationUserAccessToken;
use crate::domain_layer::data::entity::channel::Channel_Id;
use crate::domain_layer::functionality::service::application_user_access_token___extractor::ExtractorResult;
use crate::domain_layer::functionality::service::extractor::Extractor;
use crate::domain_layer::functionality::service::validator::Validator;
use crate::infrastructure_layer::data::error_auditor::BacktracePart;
use crate::infrastructure_layer::data::error_auditor::BaseError;
use crate::infrastructure_layer::data::error_auditor::ErrorAuditor;
use crate::infrastructure_layer::data::error_auditor::ResourceError;
use crate::infrastructure_layer::data::error_auditor::RuntimeError;
use crate::infrastructure_layer::data::invalid_argument_result::InvalidArgument;
use crate::infrastructure_layer::data::invalid_argument_result::InvalidArgumentResult;
use crate::infrastructure_layer::functionality::repository::common_postgresql_repository::Common1;
use crate::infrastructure_layer::functionality::repository::postgresql_repository::by::By13;
use crate::infrastructure_layer::functionality::repository::postgresql_repository::PostgresqlRepository;
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
pub use action_processor_incoming_outcoming::action_processor::channel___base::get_many_by_subscription::Incoming;
pub use action_processor_incoming_outcoming::action_processor::channel___base::get_many_by_subscription::Outcoming;

pub struct GetManyBySubscription;

impl GetManyBySubscription {
    const LIMIT: i16 = 100;

    pub async fn process<'a, T>(
        database_1_postgresql_connection_pool: &'a Pool<PostgresqlConnectionManager<T>>,
        _database_2_postgresql_connection_pool: &'a Pool<PostgresqlConnectionManager<T>>,
        _database_1_redis_connection_pool: &'a Pool<RedisConnectionManager>,
        incoming: Incoming,
    ) -> Result<InvalidArgumentResult<UnifiedReport<Outcoming, Precedent>>, ErrorAuditor>
    where
        T: MakeTlsConnect<Socket> + Clone + Send + Sync + 'static,
        <T as MakeTlsConnect<Socket>>::Stream: Send + Sync,
        <T as MakeTlsConnect<Socket>>::TlsConnect: Send,
        <<T as MakeTlsConnect<Socket>>::TlsConnect as TlsConnect<Socket>>::Future: Send,
    {
        let extractor_result = match Extractor::<ApplicationUserAccessToken<'_>>::extract(&incoming.application_user_access_token_encrypted).await {
            Ok(extractor_result_) => extractor_result_,
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

        let application_user_access_token = match extractor_result {
            InvalidArgumentResult::Ok {
                subject: extractor_result_,
            } => {
                let application_user_access_token_ = match extractor_result_ {
                    ExtractorResult::ApplicationUserAccessToken {
                        application_user_access_token: application_user_access_token__,
                    } => application_user_access_token__,
                    ExtractorResult::ApplicationUserAccessTokenAlreadyExpired => {
                        return Ok(
                            InvalidArgumentResult::Ok {
                                subject: UnifiedReport::precedent(Precedent::ApplicationUserAccessToken_AlreadyExpired),
                            },
                        );
                    }
                    ExtractorResult::ApplicationUserAccessTokenInApplicationUserAccessTokenBlackList => {
                        return Ok(
                            InvalidArgumentResult::Ok {
                                subject: UnifiedReport::precedent(Precedent::ApplicationUserAccessToken_InApplicationUserAccessTokenBlackList),
                            },
                        );
                    }
                };

                application_user_access_token_
            }
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

        if let Some(requery_channel_id_) = incoming.requery_channel_id {
            if !Validator::<Channel_Id>::is_valid(requery_channel_id_) {
                return Ok(
                    InvalidArgumentResult::InvalidArgument {
                        invalid_argument: InvalidArgument::Channel_Id,
                    },
                );
            }
        }

        if incoming.limit <= 0 || incoming.limit > Self::LIMIT {
            return Ok(
                InvalidArgumentResult::InvalidArgument {
                    invalid_argument: InvalidArgument::Limit,
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

        let common_registry = match PostgresqlRepository::<Common1>::find_3(
            &*database_1_postgresql_pooled_connection,
            &By13 {
                application_user_id: application_user_access_token.application_user_id,
                requery_channel_id: incoming.requery_channel_id,
            },
            incoming.limit,
        )
        .await
        {
            Ok(channel_registry_) => channel_registry_,
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
            common_registry,
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
        CommonPrecedent::ApplicationUserAccessToken_AlreadyExpired,
        CommonPrecedent::ApplicationUserAccessToken_InApplicationUserAccessTokenBlackList,
    }
);
