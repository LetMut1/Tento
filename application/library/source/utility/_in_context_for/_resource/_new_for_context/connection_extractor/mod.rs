use crate::error::main_error::core::run_time_error::run_time_error::RunTimeError;
use diesel::pg::PgConnection as PostgresqlConnection;
use diesel::r2d2::ConnectionManager as PostgresqlConnectionManager;
use r2d2_redis::RedisConnectionManager;
use r2d2::PooledConnection;
use std::sync::Arc;
use super::aggregate_connection_pool::AggregateConnectionPool;

pub struct ConnectionExtractor;

impl ConnectionExtractor {
    pub fn get_postgresql_connection<'outer_a>(
        aggregate_connection_pool: &'outer_a Arc<AggregateConnectionPool>
    ) -> Result<PooledConnection<PostgresqlConnectionManager<PostgresqlConnection>>, RunTimeError> {
        return Ok(aggregate_connection_pool.get_postgresql_connection_pool().get()?);
    }

    pub fn get_redis_connection<'outer_a>(
        aggregate_connection_pool: &'outer_a Arc<AggregateConnectionPool>
    ) -> Result<PooledConnection<RedisConnectionManager>, RunTimeError> {
        return Ok(aggregate_connection_pool.get_redis_connection_pool().get()?);
    }
}