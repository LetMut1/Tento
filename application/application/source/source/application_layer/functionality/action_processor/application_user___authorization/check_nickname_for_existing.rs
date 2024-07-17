use crate::{
    application_layer::{
        data::unified_report::UnifiedReport,
        functionality::action_processor::ActionProcessor,
    },
    domain_layer::{
        data::entity::application_user::{
            ApplicationUser,
            ApplicationUser_Nickname,
        },
        functionality::service::validator::Validator,
    },
    infrastructure_layer::{
        data::{
            aggregate_error::{
                AggregateError,
                Backtrace,
                OptionConverter,
                ResultConverter,
            },
            control_type::ApplicationUser__Authorization___CheckNicknameForExisting,
            environment_configuration::EnvironmentConfiguration,
            void::Void,
        },
        functionality::repository::postgresql::{
            application_user::By1,
            PostgresqlRepository,
        },
    },
};
use action_processor_incoming_outcoming::action_processor::application_user___authorization::check_nickname_for_existing::{
    Incoming,
    Outcoming,
};
use bb8::Pool;
use bb8_postgres::PostgresConnectionManager as PostgresqlConnectionManager;
use std::{
    clone::Clone,
    marker::{
        Send,
        Sync,
    },
};
use tokio_postgres::{
    tls::{
        MakeTlsConnect,
        TlsConnect,
    },
    Socket,
};
impl ActionProcessor<ApplicationUser__Authorization___CheckNicknameForExisting> {
    pub async fn process<'a, T>(
        _environment_configuration: &'a EnvironmentConfiguration,
        database_1_postgresql_connection_pool: &'a Pool<PostgresqlConnectionManager<T>>,
        _database_2_postgresql_connection_pool: &'a Pool<PostgresqlConnectionManager<T>>,
        incoming: Option<Incoming>,
    ) -> Result<UnifiedReport<Outcoming, Void>, AggregateError>
    where
        T: MakeTlsConnect<Socket> + Clone + Send + Sync + 'static,
        <T as MakeTlsConnect<Socket>>::Stream: Send + Sync,
        <T as MakeTlsConnect<Socket>>::TlsConnect: Send,
        <<T as MakeTlsConnect<Socket>>::TlsConnect as TlsConnect<Socket>>::Future: Send,
    {
        let incoming_ = incoming.into_logic_value_does_not_exist(
            Backtrace::new(
                line!(),
                file!(),
            ),
        )?;
        if !Validator::<ApplicationUser_Nickname>::is_valid(incoming_.application_user__nickname.as_str()) {
            return Err(
                AggregateError::new_invalid_argument_from_outside(
                    Backtrace::new(
                        line!(),
                        file!(),
                    ),
                ),
            );
        }
        let database_1_postgresql_pooled_connection = database_1_postgresql_connection_pool.get().await.into_runtime(
            Backtrace::new(
                line!(),
                file!(),
            ),
        )?;
        let is_exist = PostgresqlRepository::<ApplicationUser<'_>>::is_exist_1(
            &*database_1_postgresql_pooled_connection,
            By1 {
                application_user__nickname: incoming_.application_user__nickname.as_str(),
            },
        )
        .await?;
        let outcoming = Outcoming {
            result: is_exist,
        };
        return Ok(UnifiedReport::target_filled(outcoming));
    }
}
