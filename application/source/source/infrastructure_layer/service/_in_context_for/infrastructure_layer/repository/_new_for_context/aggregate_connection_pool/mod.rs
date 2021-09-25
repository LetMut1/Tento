use crate::infrastructure_layer::error::base_error::base_error::BaseError;
use crate::infrastructure_layer::service::environment_variable_resolver::EnvironmentVariableResolver;
use postgres::Config;
use postgres::NoTls;
use r2d2_postgres::PostgresConnectionManager as PostgresqlConnectionManager;
use r2d2_redis::RedisConnectionManager;
use r2d2::Pool;
use redis::ConnectionInfo;
use std::clone::Clone;
use std::str::FromStr;

#[derive(Clone)]
pub struct AggregateConnectionPool {
    postgresql_connection_pool: Pool<PostgresqlConnectionManager<NoTls>>, // TODO Для девелопмента ТЛС не нужен (НО можно подключить, как вариант), для Продакша - обязательно. Здесь Пул, который содержит только для Дев. Можно Пулы выделить в Оптион для дев и прод окруженияю. Либо через Дженерик, создавать и отдавать в зависимости от от ИзПродакшн значения. Либо Base it on a feature? Probably having NoTls be the feature, since it makes more sense to have TLS by default
    redis_connection_pool: Pool<RedisConnectionManager>
}

impl AggregateConnectionPool {
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
    ) -> &'a Pool<PostgresqlConnectionManager<NoTls>> {
        return &self.postgresql_connection_pool;
    }

    pub fn get_redis_connection_pool<'a>(
        &'a self
    ) -> &'a Pool<RedisConnectionManager> {
        return &self.redis_connection_pool;
    }

    fn establish_postgresql_connection_pool(
    ) -> Result<Pool<PostgresqlConnectionManager<NoTls>>, BaseError> { // TODO create Pool with builder in preProd state. Просчитать, какое количство Threads можнт использовать одновременно для Actix. Создать с tls
        return Ok(
            Pool::new(
                PostgresqlConnectionManager::new(
                    Config::from_str(EnvironmentVariableResolver::get_resource_postgresql_url()?.as_str())?,
                    NoTls
                )
            )?
        );
    }

    fn establish_redis_connection_pool(
    ) -> Result<Pool<RedisConnectionManager>, BaseError> { // TODO create Pool with builder in preProd state. Просчитать, какое количство Threads можнт использовать одновременно для Actix
        return Ok(
            Pool::new(
                RedisConnectionManager::new(
                    ConnectionInfo::from_str(EnvironmentVariableResolver::get_resource_redis_url()?.as_str())?
                )?
            )?
        );
    }
}