use crate::application_layer::data::unified_report::UnifiedReport;
use crate::domain_layer::data::entity::application_user::ApplicationUser;
use crate::domain_layer::data::entity::application_user::ApplicationUser_Nickname;
use crate::domain_layer::functionality::service::validator::Validator;
use crate::infrastructure_layer::data::auditor::BacktracePart;
use crate::infrastructure_layer::data::error::Error;
use crate::infrastructure_layer::data::auditor::Auditor;
use crate::infrastructure_layer::data::auditor::Converter;
use crate::infrastructure_layer::data::invalid_argument_result::InvalidArgument;
use crate::infrastructure_layer::data::invalid_argument_result::InvalidArgumentResult;
use crate::infrastructure_layer::data::void::Void;
use crate::infrastructure_layer::functionality::repository::postgresql::by::By1;
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

pub use action_processor_incoming_outcoming::action_processor::application_user___authorization::check_nickname_for_existing::Incoming;
pub use action_processor_incoming_outcoming::action_processor::application_user___authorization::check_nickname_for_existing::Outcoming;
pub use crate::infrastructure_layer::data::control_type::ApplicationUser__Authorization___CheckNicknameForExisting;

impl ActionProcessor<ApplicationUser__Authorization___CheckNicknameForExisting> {
    pub async fn process<'a, T>(
        database_1_postgresql_connection_pool: &'a Pool<PostgresqlConnectionManager<T>>,
        _database_2_postgresql_connection_pool: &'a Pool<PostgresqlConnectionManager<T>>,
        _database_1_redis_connection_pool: &'a Pool<RedisConnectionManager>,
        incoming: Option<Incoming>,
    ) -> Result<InvalidArgumentResult<UnifiedReport<Outcoming, Void>>, Auditor<Error>>
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

        if !Validator::<ApplicationUser_Nickname>::is_valid(&incoming_.application_user_nickname) {
            return Ok(
                InvalidArgumentResult::InvalidArgument {
                    invalid_argument: InvalidArgument::ApplicationUser_Nickname,
                },
            );
        }

        let database_1_postgresql_pooled_connection = database_1_postgresql_connection_pool.get().await.convert(BacktracePart::new(line!(), file!()))?;

        let is_exist = PostgresqlRepository::<ApplicationUser<'_>>::is_exist_1(
            &*database_1_postgresql_pooled_connection,
            &By1 {
                application_user_nickname: &incoming_.application_user_nickname,
            },
        )
        .await?;

        let outcoming = Outcoming {
            result: is_exist,
        };

        return Ok(
            InvalidArgumentResult::Ok {
                subject: UnifiedReport::target_filled(outcoming),
            },
        );
    }
}