use crate::infrastructure_layer::data::environment_configuration::Environment;
use crate::infrastructure_layer::data::error_auditor::BacktracePart;
use crate::infrastructure_layer::data::error_auditor::BaseError;
use crate::infrastructure_layer::data::error_auditor::ErrorAuditor;
use crate::infrastructure_layer::data::error_auditor::ResourceError;
use crate::infrastructure_layer::data::error_auditor::RuntimeError;
use extern_crate::bb8_postgres::PostgresConnectionManager as PostgresqlConnectionManager;
use extern_crate::bb8_redis::RedisConnectionManager;
use extern_crate::bb8::Pool;
use extern_crate::redis::ConnectionInfo;
use extern_crate::tokio_postgres::config::Config;
use extern_crate::tokio_postgres::NoTls;
use std::clone::Clone;
use std::marker::PhantomData;

pub struct Creator<S> {
    _subject: PhantomData<S>
}

pub type PostgresqlConnectionPoolNoTls = Pool<PostgresqlConnectionManager<NoTls>>;

impl Creator<PostgresqlConnectionPoolNoTls> {
    pub async fn create<'a>(environment: &'a Environment, configuration: &'a Config) -> Result<Pool<PostgresqlConnectionManager<NoTls>>, ErrorAuditor> {
        let postgresql_connection_pool = match *environment {
            Environment::Production => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::LogicError { message: "NoTls should be only not in production environment." },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
            Environment::Development |
            Environment::LocalDevelopment => {
                let postgresql_connection_pool_ = match Pool::builder()
                    .build(
                        PostgresqlConnectionManager::new(configuration.clone(), NoTls)
                    ).await {
                    Ok(postgresql_connection_pool__) => postgresql_connection_pool__,
                    Err(error) => {
                        return Err(
                            ErrorAuditor::new(
                                BaseError::RuntimeError { runtime_error: RuntimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                                BacktracePart::new(line!(), file!(), None)
                            )
                        );
                    }
                };

                postgresql_connection_pool_
            }
        };

        return Ok(postgresql_connection_pool);
    }
}

pub type RedisConnectonPool = Pool<RedisConnectionManager>;

impl Creator<RedisConnectonPool> {
    pub async fn create<'a>(environment: &'a Environment, connection_info: &'a ConnectionInfo) -> Result<Pool<RedisConnectionManager>, ErrorAuditor> {
        let redis_connection_pool = match *environment {
            Environment::Production => {
                todo!();
            }
            Environment::Development |
            Environment::LocalDevelopment => {
                let redis_connection_manager = match RedisConnectionManager::new(connection_info.clone()) {
                    Ok(redis_connection_manager_) => redis_connection_manager_,
                    Err(error) => {
                        return Err(
                            ErrorAuditor::new(
                                BaseError::RuntimeError { runtime_error: RuntimeError::ResourceError { resource_error: ResourceError::RedisError { redis_error: error } } },
                                BacktracePart::new(line!(), file!(), None)
                            )
                        );
                    }
                };

                let redis_connection_pool_ = match Pool::builder()      // TODO TODO TODO TODO TODO create Pool with builder in preProd state. НАСТРОИТТЬ ПУУЛ
                    .build(redis_connection_manager)
                    .await {
                    Ok(redis_connection_pool__) => redis_connection_pool__,
                    Err(error) => {
                        return Err(
                            ErrorAuditor::new(
                                BaseError::RuntimeError { runtime_error: RuntimeError::ResourceError { resource_error: ResourceError::RedisError { redis_error: error } } },
                                BacktracePart::new(line!(), file!(), None)
                            )
                        );
                    }
                };

                redis_connection_pool_
            }
        };

        return Ok(redis_connection_pool);
    }
}