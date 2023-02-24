use crate::infrastructure_layer::data::environment_configuration::Environment;
use crate::infrastructure_layer::data::error_auditor::BacktracePart;
use crate::infrastructure_layer::data::error_auditor::BaseError;
use crate::infrastructure_layer::data::error_auditor::ErrorAuditor;
use crate::infrastructure_layer::data::error_auditor::ResourceError;
use crate::infrastructure_layer::data::error_auditor::RuntimeError;
use extern_crate::bb8_postgres::PostgresConnectionManager as PostgresqlConnectionManager;
use extern_crate::bb8::Pool;
use extern_crate::tokio_postgres::config::Config;
use extern_crate::tokio_postgres::NoTls;
use std::clone::Clone;
use std::marker::PhantomData;

pub struct PostgresqlConnectionPoolCreator<T> {
    _tls_connection: PhantomData<T>
}

impl PostgresqlConnectionPoolCreator<NoTls> {
    pub async fn create<'a>(environment: &'a Environment, configuraion: &'a Config) -> Result<Pool<PostgresqlConnectionManager<NoTls>>, ErrorAuditor> {
        match *environment {
            Environment::Production => {
                todo!();
            }
            Environment::Development |
            Environment::LocalDevelopment => {
                return match Pool::builder()
                    .build(
                        PostgresqlConnectionManager::new(configuraion.clone(), NoTls)
                    ).await {
                    Ok(database_1_postgresql_connection_pool_) => Ok(database_1_postgresql_connection_pool_),
                    Err(error) => {
                        return Err(
                            ErrorAuditor::new(
                                BaseError::RuntimeError { runtime_error: RuntimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                                BacktracePart::new(line!(), file!(), None)
                            )
                        );
                    }
                }
            }
        }
    }
}