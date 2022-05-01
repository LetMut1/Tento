use bb8_postgres::PostgresConnectionManager as PostgresqlConnectionManager;
use bb8::Pool;
use std::clone::Clone;
use tokio_postgres::NoTls;

#[derive(Clone)]
pub enum PostgresqlConnectionPool {
    Development {
        postgresql_connection_pool: Pool<PostgresqlConnectionManager<NoTls>>
    }
}