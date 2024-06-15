use crate::application_layer::data::unified_report::UnifiedReport;
use crate::application_layer::functionality::action_processor::ActionProcessor;
use crate::infrastructure_layer::data::auditor::Auditor;
pub use crate::infrastructure_layer::data::control_type::HealthCheck;
use crate::infrastructure_layer::data::environment_configuration::EnvironmentConfiguration;
use crate::infrastructure_layer::data::error::Error;
use crate::infrastructure_layer::data::invalid_argument::InvalidArgument;
use crate::infrastructure_layer::data::void::Void;
use bb8::Pool;
use bb8_postgres::PostgresConnectionManager as PostgresqlConnectionManager;
use std::clone::Clone;
use std::marker::Send;
use std::marker::Sync;
use tokio_postgres::tls::MakeTlsConnect;
use tokio_postgres::tls::TlsConnect;
use tokio_postgres::Socket;
impl ActionProcessor<HealthCheck> {
    pub async fn process<'a, T>(
        _environment_configuration: &'a EnvironmentConfiguration,
        _database_1_postgresql_connection_pool: &'a Pool<PostgresqlConnectionManager<T>>,
        _database_2_postgresql_connection_pool: &'a Pool<PostgresqlConnectionManager<T>>,
        _incoming: Option<Void>,
    ) -> Result<Result<UnifiedReport<Void, Void>, Auditor<InvalidArgument>>, Auditor<Error>>
    where
        T: MakeTlsConnect<Socket> + Clone + Send + Sync + 'static,
        <T as MakeTlsConnect<Socket>>::Stream: Send + Sync,
        <T as MakeTlsConnect<Socket>>::TlsConnect: Send,
        <<T as MakeTlsConnect<Socket>>::TlsConnect as TlsConnect<Socket>>::Future: Send,
    {
        todo!();
    }
}
