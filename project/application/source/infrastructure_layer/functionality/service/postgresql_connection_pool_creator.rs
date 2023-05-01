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
use super::creator::Creator;

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