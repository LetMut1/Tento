use bb8_postgres::PostgresConnectionManager as PostgresqlConnectionManager;
use bb8_redis::RedisConnectionManager;
use bb8::Pool;
use crate::infrastructure_layer::error::base_error::base_error::BaseError;
use crate::infrastructure_layer::service::environment_variable_resolver::EnvironmentVariableResolver;
use postgres::Config as ConfigXXXxDelete;
use postgres::NoTls as NoTlsXXXxDelete;
use r2d2_postgres::PostgresConnectionManager as PostgresqlConnectionManagerXXXxDelete;
use r2d2_redis::RedisConnectionManager as RedisConnectionManagerXXXxDelete;
use r2d2::Pool as PoolXXXxDelete;
use redis_ref::ConnectionInfo;
use redis::ConnectionInfo as ConnectionInfoXXXxDelete;
use std::clone::Clone;
use std::str::FromStr;
use tokio_postgres::Config;
use tokio_postgres::NoTls;

#[derive(Clone)]
pub struct AggregateConnectionPoolXXXxDELETE {
    postgresql_connection_pool: PoolXXXxDelete<PostgresqlConnectionManagerXXXxDelete<NoTls>>, // TODO Для девелопмента ТЛС не нужен (НО можно подключить, как вариант), для Продакша - обязательно. Здесь Пул, который содержит только для Дев. Можно Пулы выделить в Оптион для дев и прод окруженияю. Либо через Дженерик, создавать и отдавать в зависимости от от ИзПродакшн значения. Либо Base it on a feature? Probably having NoTls be the feature, since it makes more sense to have TLS by default
    redis_connection_pool: PoolXXXxDelete<RedisConnectionManagerXXXxDelete>
}

impl AggregateConnectionPoolXXXxDELETE {
    pub fn new(
    ) -> Result<Self, BaseError> {
        return Ok (
            Self {
                postgresql_connection_pool: Self::establish_postgresql_connection_pool()?,
                redis_connection_pool: Self::establish_redis_connection_pool()?
            }
        );
    }

    pub fn get_postgresql_connection_pool<'a>(
        &'a self
    ) -> &'a PoolXXXxDelete<PostgresqlConnectionManagerXXXxDelete<NoTls>> {
        return &self.postgresql_connection_pool;
    }

    pub fn get_redis_connection_pool<'a>(
        &'a self
    ) -> &'a PoolXXXxDelete<RedisConnectionManagerXXXxDelete> {
        return &self.redis_connection_pool;
    }

    fn establish_postgresql_connection_pool(
    ) -> Result<PoolXXXxDelete<PostgresqlConnectionManagerXXXxDelete<NoTlsXXXxDelete>>, BaseError> { // TODO create Pool with builder in preProd state. Просчитать, какое количство Threads можнт использовать одновременно для Actix. Создать с tls
        return Ok(
            PoolXXXxDelete::new(
                PostgresqlConnectionManagerXXXxDelete::new(
                    ConfigXXXxDelete::from_str(EnvironmentVariableResolver::get_resource_postgresql_url()?.as_str())?,
                    NoTlsXXXxDelete
                )
            )?
        );
    }

    fn establish_redis_connection_pool(
    ) -> Result<PoolXXXxDelete<RedisConnectionManagerXXXxDelete>, BaseError> { // TODO create Pool with builder in preProd state. Просчитать, какое количство Threads можнт использовать одновременно для Actix
        return Ok(
            PoolXXXxDelete::new(
                RedisConnectionManagerXXXxDelete::new(
                    ConnectionInfoXXXxDelete::from_str(EnvironmentVariableResolver::get_resource_redis_url()?.as_str())?
                )?
            )?
        );
    }
}

#[derive(Clone)]
pub struct AggregateConnectionPool {
    postgresql_connection_pool: Pool<PostgresqlConnectionManager<NoTls>>, // TODO Для девелопмента ТЛС не нужен (НО можно подключить, как вариант), для Продакша - обязательно. Здесь Пул, который содержит только для Дев. Можно Пулы выделить в Оптион для дев и прод окруженияю. Либо через Дженерик, создавать и отдавать в зависимости от от ИзПродакшн значения. Либо Base it on a feature? Probably having NoTls be the feature, since it makes more sense to have TLS by default
    redis_connection_pool: Pool<RedisConnectionManager>
}

impl AggregateConnectionPool {
    pub async fn new(
    ) -> Result<Self, BaseError> {
        return Ok (
            Self {
                postgresql_connection_pool: Self::establish_postgresql_connection_pool().await?,
                redis_connection_pool: Self::establish_redis_connection_pool().await?
            }
        );
    }

    pub fn get_postgresql_connection_pool<'a>(
        &'a self
    ) -> &'a Pool<PostgresqlConnectionManager<NoTls>> {
        return &self.postgresql_connection_pool;
    }

    pub fn get_redis_connection_pool<'a>(
        &'a self
    ) -> &'a Pool<RedisConnectionManager> {
        return &self.redis_connection_pool;
    }

    async fn establish_postgresql_connection_pool(
    ) -> Result<Pool<PostgresqlConnectionManager<NoTls>>, BaseError> { // TODO TODO TODO TODO TODO create Pool with builder in preProd state. НАСТРОИТТЬ ПУУЛ
        return Ok(
            Pool::builder().build(
                PostgresqlConnectionManager::new(
                    Config::from_str(EnvironmentVariableResolver::get_resource_postgresql_url()?.as_str())?,
                    NoTls
                )
            ).await?
        );
    }

    async fn establish_redis_connection_pool(
    ) -> Result<Pool<RedisConnectionManager>, BaseError> { // TODO TODO TODO TODO TODO create Pool with builder in preProd state. НАСТРОИТТЬ ПУУЛ
        return Ok(
            Pool::builder().build(
                RedisConnectionManager::new(
                    ConnectionInfo::from_str(EnvironmentVariableResolver::get_resource_redis_url()?.as_str())?
                )?
            ).await?
        );
    }
}