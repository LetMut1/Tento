use crate::{
    application_layer::{
        data::unified_report::UnifiedReport,
        functionality::action_processor::ActionProcessor,
    },
    domain_layer::{
        data::entity::application_user::{
            ApplicationUser,
            ApplicationUser_Email,
        },
        functionality::service::validator::Validator,
    },
    infrastructure_layer::{
        data::{
            aggregate_error::{
                AggregateError,
                Backtrace,
                ResultConverter,
            },
            control_type::ApplicationUser__Authorization___CheckEmailForExisting,
            environment_configuration::EnvironmentConfiguration,
            void::Void,
        },
        functionality::repository::postgresql::{
            application_user::By2,
            PostgresqlRepository,
        },
    },
};
use action_processor_incoming_outcoming::action_processor::application_user___authorization::check_email_for_existing::{
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
impl ActionProcessor<ApplicationUser__Authorization___CheckEmailForExisting> {
    pub async fn process<'a, T>(
        _environment_configuration: &'a EnvironmentConfiguration,
        database_1_postgresql_connection_pool: &'a Pool<PostgresqlConnectionManager<T>>,
        _database_2_postgresql_connection_pool: &'a Pool<PostgresqlConnectionManager<T>>,
        incoming: Incoming,
    ) -> Result<UnifiedReport<Outcoming, Void>, AggregateError>
    where
        T: MakeTlsConnect<Socket> + Clone + Send + Sync + 'static,
        <T as MakeTlsConnect<Socket>>::Stream: Send + Sync,
        <T as MakeTlsConnect<Socket>>::TlsConnect: Send,
        <<T as MakeTlsConnect<Socket>>::TlsConnect as TlsConnect<Socket>>::Future: Send,
    {
        if !Validator::<ApplicationUser_Email>::is_valid(incoming.application_user__email.as_str())? {
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
        let is_exist = PostgresqlRepository::<ApplicationUser<'_>>::is_exist_2(
            &*database_1_postgresql_pooled_connection,
            By2 {
                application_user__email: incoming.application_user__email.as_str(),
            },
        )
        .await?;
        let outcoming = Outcoming {
            result: is_exist,
        };
        return Ok(UnifiedReport::target_filled(outcoming));
    }
}
