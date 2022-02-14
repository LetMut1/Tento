use bb8_postgres::PostgresConnectionManager as PostgresqlConnectionManager;
use bb8_redis::RedisConnectionManager;
use bb8::PooledConnection;
use crate::infrastructure_layer::error::base_error::base_error::BaseError;
use postgres::NoTls as NoTlsXXXxDelete;
use r2d2_postgres::PostgresConnectionManager as PostgresqlConnectionManagerXXXxDelete;
use r2d2_redis::RedisConnectionManager as RedisConnectionManagerXXXxDelete;
use r2d2::PooledConnection as PooledConnectionXXXxDelete;
use std::sync::Arc;
use super::aggregate_connection_pool::AggregateConnectionPool;
use super::aggregate_connection_pool::AggregateConnectionPoolXXXxDELETE;
use tokio_postgres::NoTls;

pub struct ConnectionExtractorXXXxDelete;

impl ConnectionExtractorXXXxDelete {
    pub fn get_postgresql_connection<'a>(
        aggregate_connection_pool: &'a Arc<AggregateConnectionPoolXXXxDELETE>
    ) -> Result<PooledConnectionXXXxDelete<PostgresqlConnectionManagerXXXxDelete<NoTlsXXXxDelete>>, BaseError> {  // TODO NoTls-problem 
        return Ok(aggregate_connection_pool.get_postgresql_connection_pool().get()?);
    }

    pub fn get_redis_connection<'a>(
        aggregate_connection_pool: &'a Arc<AggregateConnectionPoolXXXxDELETE>
    ) -> Result<PooledConnectionXXXxDelete<RedisConnectionManagerXXXxDelete>, BaseError> {
        return Ok(aggregate_connection_pool.get_redis_connection_pool().get()?);
    }
}

pub struct ConnectionExtractor;

impl ConnectionExtractor {
    pub async fn get_postgresql_connection<'a>(
        aggregate_connection_pool: &'a Arc<AggregateConnectionPool>
    ) -> Result<PooledConnection<'_, PostgresqlConnectionManager<NoTls>>, BaseError> {  // TODO NoTls-problem 
        return Ok(aggregate_connection_pool.get_postgresql_connection_pool().get().await?);
    }

    pub async fn get_redis_connection<'a>(
        aggregate_connection_pool: &'a Arc<AggregateConnectionPool>
    ) -> Result<PooledConnection<'_, RedisConnectionManager>, BaseError> {
        return Ok(aggregate_connection_pool.get_redis_connection_pool().get().await?);
    }
}