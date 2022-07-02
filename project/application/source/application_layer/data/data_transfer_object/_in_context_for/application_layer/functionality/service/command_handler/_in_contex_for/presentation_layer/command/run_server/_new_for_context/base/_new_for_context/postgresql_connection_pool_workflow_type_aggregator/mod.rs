use bb8_postgres::PostgresConnectionManager as PostgresqlConnectionManager;
use bb8::Pool;
use std::clone::Clone;
use tokio_postgres::NoTls;

#[derive(Clone)]
pub enum PostgresqlConnectionPoolWorkflowTypeAggregator {
    LocalDevelopment {
        core_postgresql_connection_pool: Pool<PostgresqlConnectionManager<NoTls>>,
        authorization_postgresql_connection_pool: Pool<PostgresqlConnectionManager<NoTls>>
    }
}