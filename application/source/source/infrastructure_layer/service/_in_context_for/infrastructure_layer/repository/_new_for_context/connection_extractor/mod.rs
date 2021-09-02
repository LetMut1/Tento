use crate::infrastructure_layer::error::base_error::base_error::BaseError;
use diesel::pg::PgConnection as PostgresqlXXXDELETEConnection;
use diesel::r2d2::ConnectionManager as PostgresqlXXXDELETEConnectionManager;
use postgres::NoTls;
use r2d2_postgres::PostgresConnectionManager as PostgresqlConnectionManager;
use r2d2_redis::RedisConnectionManager;
use r2d2::PooledConnection;
use std::sync::Arc;
use super::aggregate_connection_pool::AggregateConnectionPool;

pub struct ConnectionExtractor;

impl ConnectionExtractor {
    pub fn get_postgresqlxxxdelete_connection<'outer_a>(
        aggregate_connection_pool: &'outer_a Arc<AggregateConnectionPool>
    ) -> Result<PooledConnection<PostgresqlXXXDELETEConnectionManager<PostgresqlXXXDELETEConnection>>, BaseError> {
        return Ok(aggregate_connection_pool.get_postgresqlxxxxxxx_connection_pool().get()?);
    }

    pub fn get_postgresql_connection<'outer_a>(
        aggregate_connection_pool: &'outer_a Arc<AggregateConnectionPool>
    ) -> Result<PooledConnection<PostgresqlConnectionManager<NoTls>>, BaseError> {  // TODO NoTls-problem 
        return Ok(aggregate_connection_pool.get_postgresql_connection_pool().get()?);
    }

    pub fn get_redis_connection<'outer_a>(
        aggregate_connection_pool: &'outer_a Arc<AggregateConnectionPool>
    ) -> Result<PooledConnection<RedisConnectionManager>, BaseError> {
        return Ok(aggregate_connection_pool.get_redis_connection_pool().get()?);
    }
}