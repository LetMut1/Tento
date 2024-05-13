use crate::application_layer::data::unified_report::UnifiedReport;
use crate::domain_layer::data::entity::application_user_access_refresh_token::ApplicationUserAccessRefreshToken;
use crate::domain_layer::data::entity::application_user_access_token::ApplicationUserAccessToken;
use crate::domain_layer::functionality::service::extractor::application_user_access_token::ExtractorResult;
use crate::domain_layer::functionality::service::extractor::Extractor;
use crate::infrastructure_layer::data::auditor::Backtrace;
use crate::infrastructure_layer::data::error::Error;
use crate::infrastructure_layer::data::auditor::Auditor;
use crate::infrastructure_layer::data::auditor::OptionConverter;
use crate::infrastructure_layer::data::auditor::ErrorConverter;
use crate::infrastructure_layer::data::invalid_argument_result::InvalidArgumentResult;
use crate::infrastructure_layer::data::void::Void;
use crate::infrastructure_layer::functionality::repository::postgresql::by::By4;
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

pub use action_processor_incoming_outcoming::action_processor::application_user___authorization::deauthorize_from_one_device::Incoming;
pub use action_processor_incoming_outcoming::action_processor::application_user___authorization::deauthorize_from_one_device::Precedent;
pub use crate::infrastructure_layer::data::control_type::ApplicationUser__Authorization___DeauthorizeFromOneDevice;


impl ActionProcessor<ApplicationUser__Authorization___DeauthorizeFromOneDevice> {
    pub async fn process<'a, T>(
        _database_1_postgresql_connection_pool: &'a Pool<PostgresqlConnectionManager<T>>,
        database_2_postgresql_connection_pool: &'a Pool<PostgresqlConnectionManager<T>>,
        _database_1_redis_connection_pool: &'a Pool<RedisConnectionManager>,
        incoming: Option<Incoming>,
    ) -> Result<InvalidArgumentResult<UnifiedReport<Void, Precedent>>, Auditor<Error>>
    where
        T: MakeTlsConnect<Socket> + Clone + Send + Sync + 'static,
        <T as MakeTlsConnect<Socket>>::Stream: Send + Sync,
        <T as MakeTlsConnect<Socket>>::TlsConnect: Send,
        <<T as MakeTlsConnect<Socket>>::TlsConnect as TlsConnect<Socket>>::Future: Send,
    {
        let incoming_ = incoming.convert_value_should_exist(Backtrace::new(line!(), file!()))?;

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

        let database_2_postgresql_pooled_connection = database_2_postgresql_connection_pool.get().await.convert(Backtrace::new(line!(), file!()))?;

        PostgresqlRepository::<ApplicationUserAccessRefreshToken<'_>>::delete_1(
            &*database_2_postgresql_pooled_connection,
            &By4 {
                application_user_id: application_user_access_token.application_user_id,
                application_user_device_id: application_user_access_token.application_user_device_id.as_ref(),
            },
        )
        .await?;

        return Ok(
            InvalidArgumentResult::Ok {
                subject: UnifiedReport::target_empty(),
            },
        );
    }
}
