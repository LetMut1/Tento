use crate::error::main_error_kind::core::resource_error_kind::resource_error_kind::ResourceErrorKind;
use diesel::pg::PgConnection as PostgresqlConnection;
use diesel::r2d2::ConnectionManager as PostgresqlConnectionManager;
use r2d2_redis::RedisConnectionManager;
use r2d2::Pool;
use std::clone::Clone;

#[derive(Clone)]
pub struct AggregateConnectionPool {
    postgresql_connection_pool: Pool<PostgresqlConnectionManager<PostgresqlConnection>>,
    redis_connection_pool: Pool<RedisConnectionManager>
}

impl<'this> AggregateConnectionPool {
    pub fn new() -> Result<Self, ResourceErrorKind> {
        return Ok (
            Self {
                postgresql_connection_pool: Self::establish_postgresql_connection_pool()?,
                redis_connection_pool: Self::establish_redis_connection_pool()?
            }
        );
    }

    fn establish_postgresql_connection_pool() -> Result<Pool<PostgresqlConnectionManager<PostgresqlConnection>>, ResourceErrorKind> {
        return Ok(
            Pool::new(PostgresqlConnectionManager::<PostgresqlConnection>::new("postgres://root:password@postgresql/mem_is"))?       // TODO from .env;
        );   // TODO create Pool with builder in preProd state. Просчитать, какое количство Threads можнт использовать одновременно для Actix
    }

    fn establish_redis_connection_pool() -> Result<Pool<RedisConnectionManager>, ResourceErrorKind> {
        return Ok(
            Pool::new(RedisConnectionManager::new("redis://redis")?)?       // TODO from .env;
        );   // TODO create Pool with builder in preProd state. Просчитать, какое количство Threads можнт использовать одновременно для Actix
    }

    pub fn get_postgresql_connection_pool(&'this self) -> &'this Pool<PostgresqlConnectionManager<PostgresqlConnection>> {
        return &self.postgresql_connection_pool;
    }

    pub fn get_redis_connection_pool(&'this self) -> &'this Pool<RedisConnectionManager> {
        return &self.redis_connection_pool;
    }
}