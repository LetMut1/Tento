use bb8_postgres::PostgresConnectionManager as PostgresqlConnectionManager;
use bb8::Pool;
use std::clone::Clone;
use tokio_postgres::NoTls;

#[derive(Clone)]
pub enum PostgresqlConnectionPool {
    LocalDevelopment {
        postgresql_core_connection_pool: Pool<PostgresqlConnectionManager<NoTls>>,
        postgresql_authorization_connection_pool: Pool<PostgresqlConnectionManager<NoTls>>
    }
}