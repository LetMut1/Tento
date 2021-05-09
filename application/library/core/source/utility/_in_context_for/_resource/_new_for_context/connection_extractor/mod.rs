use crate::error::main_error_kind::core::resource_error_kind::resource_error_kind::ResourceErrorKind;
use diesel::pg::PgConnection as PostgresqlConnection;
use diesel::r2d2::ConnectionManager as PostgresqlConnectionManager;
use r2d2::PooledConnection;
use r2d2_redis::RedisConnectionManager;
use std::sync::Arc;
use super::aggregate_connection_pool::AggregateConnectionPool;

pub struct ConnectionExtractor;

impl ConnectionExtractor {
    pub fn get_postgresql_connection(
        aggregate_connection_pool: Arc<AggregateConnectionPool>
    ) -> Result<PooledConnection<PostgresqlConnectionManager<PostgresqlConnection>>, ResourceErrorKind> {
        return Ok(aggregate_connection_pool.get_postgresql_connection_pool().get()?);
    }
}