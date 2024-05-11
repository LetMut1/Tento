use crate::application_layer::data::unified_report::UnifiedReport;
use crate::domain_layer::data::entity::application_user_access_token::ApplicationUserAccessToken;
use crate::domain_layer::data::entity::channel::Channel_Name;
use crate::domain_layer::functionality::service::extractor::application_user_access_token::ExtractorResult;
use crate::domain_layer::functionality::service::extractor::Extractor;
use crate::domain_layer::functionality::service::validator::Validator;
use crate::infrastructure_layer::data::auditor::BacktracePart;
use crate::infrastructure_layer::data::error::Error;
use crate::infrastructure_layer::data::auditor::Auditor;
use crate::infrastructure_layer::data::auditor::Converter;
use crate::infrastructure_layer::data::invalid_argument_result::InvalidArgument;
use crate::infrastructure_layer::data::invalid_argument_result::InvalidArgumentResult;
use crate::infrastructure_layer::functionality::repository::postgresql::common::Common1;
use crate::infrastructure_layer::functionality::repository::postgresql::by::By12;
use crate::infrastructure_layer::functionality::repository::postgresql::PostgresqlRepository;
use bb8::Pool;
use bb8_postgres::PostgresConnectionManager as PostgresqlConnectionManager;
use bb8_redis::RedisConnectionManager;
use std::clone::Clone;
use std::marker::Send;
use std::marker::Sync;
use tokio_postgres::tls::MakeTlsConnect;
use tokio_postgres::tls::TlsConnect;
use tokio_postgres::Socket;
use crate::application_layer::functionality::action_processor::ActionProcessor;

pub use action_processor_incoming_outcoming::action_processor::channel___base::get_many_by_name_in_subscriptions::Incoming;
pub use action_processor_incoming_outcoming::action_processor::channel___base::get_many_by_name_in_subscriptions::Outcoming;
pub use action_processor_incoming_outcoming::action_processor::channel___base::get_many_by_name_in_subscriptions::Precedent;
pub use crate::infrastructure_layer::data::control_type::Channel__Base___GetManyByNameInSubscriptions;

impl ActionProcessor<Channel__Base___GetManyByNameInSubscriptions> {
    const LIMIT: i16 = 100;

    pub async fn process<'a, T>(
        database_1_postgresql_connection_pool: &'a Pool<PostgresqlConnectionManager<T>>,
        _database_2_postgresql_connection_pool: &'a Pool<PostgresqlConnectionManager<T>>,
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
                        Error::new_logic_incoming_invalid_state(),
                        BacktracePart::new(
                            line!(),
                            file!(),
                        ),
                    ),
                );
            }
        };

        let application_user_access_token = match Extractor::<ApplicationUserAccessToken<'_>>::extract(&incoming_.application_user_access_token_encrypted).await? {
            InvalidArgumentResult::Ok {
                subject: extractor_result,
            } => {
                let application_user_access_token_ = match extractor_result {
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

        if incoming_.limit <= 0 || incoming_.limit > Self::LIMIT {
            return Ok(
                InvalidArgumentResult::InvalidArgument {
                    invalid_argument: InvalidArgument::Limit,
                },
            );
        }

        if !Validator::<Channel_Name>::is_valid(&incoming_.channel_name) {
            return Ok(
                InvalidArgumentResult::InvalidArgument {
                    invalid_argument: InvalidArgument::Channel_Name,
                },
            );
        }

        if let Some(ref requery_channel_name_) = incoming_.requery_channel_name {
            if !Validator::<Channel_Name>::is_valid(requery_channel_name_) {
                return Ok(
                    InvalidArgumentResult::InvalidArgument {
                        invalid_argument: InvalidArgument::Channel_Name,
                    },
                );
            }
        }

        let database_1_postgresql_pooled_connection = database_1_postgresql_connection_pool.get().await.convert(BacktracePart::new(line!(), file!()))?;

        let common_registry = PostgresqlRepository::<Common1>::find_2(
            &*database_1_postgresql_pooled_connection,
            &By12 {
                application_user_id: application_user_access_token.application_user_id,
                channel_name: &incoming_.channel_name,
                requery_channel_name: &incoming_.requery_channel_name,
            },
            incoming_.limit,
        )
        .await?;

        let outcoming = Outcoming {
            common_registry,
        };

        return Ok(
            InvalidArgumentResult::Ok {
                subject: UnifiedReport::target_filled(outcoming),
            },
        );
    }
}
