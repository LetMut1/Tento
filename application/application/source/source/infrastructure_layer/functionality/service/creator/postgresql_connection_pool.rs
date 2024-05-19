use super::Creator;
use crate::infrastructure_layer::data::environment_configuration::Environment;
use crate::infrastructure_layer::data::auditor::Backtrace;
use crate::infrastructure_layer::data::environment_configuration::EnvironmentConfiguration;
use crate::infrastructure_layer::data::error::Error;
use crate::infrastructure_layer::data::auditor::Auditor;
use crate::infrastructure_layer::data::auditor::ErrorConverter;
use std::str::FromStr;
use bb8::Pool;
use bb8_postgres::PostgresConnectionManager as PostgresqlConnectionManager;
use std::clone::Clone;
use tokio_postgres::config::Config;
use tokio_postgres::NoTls;

pub use crate::infrastructure_layer::data::control_type::PostgresqlConnectionPoolNoTls;

impl Creator<PostgresqlConnectionPoolNoTls> {
    pub async fn create_database_1<'a>(
        environment_configuration: &'a EnvironmentConfiguration,
    ) -> Result<Pool<PostgresqlConnectionManager<NoTls>>, Auditor<Error>> {
        return Ok(
            Self::create(
                &environment_configuration.environment,
                &Config::from_str(environment_configuration.resource.postgresql.database_1_url.as_str()).convert(Backtrace::new(line!(), file!()))?
            ).await?
        );
    }

    pub async fn create_database_2<'a>(
        environment_configuration: &'a EnvironmentConfiguration,
    ) -> Result<Pool<PostgresqlConnectionManager<NoTls>>, Auditor<Error>> {
        return Ok(
            Self::create(
                &environment_configuration.environment,
                &Config::from_str(environment_configuration.resource.postgresql.database_2_url.as_str()).convert(Backtrace::new(line!(), file!()))?
            ).await?
        );
    }

    async fn create<'a>(
        environment: &'a Environment,
        configuration: &'a Config,
    ) -> Result<Pool<PostgresqlConnectionManager<NoTls>>, Auditor<Error>> {
        let postgresql_connection_pool = match *environment {
            Environment::Production => {
                return Err(
                    Auditor::<Error>::new(
                        Error::Logic {
                            message: "NoTls should be only not in production environment.",
                        },
                        Backtrace::new(
                            line!(),
                            file!(),
                        ),
                    ),
                );
            }
            Environment::Development | Environment::LocalDevelopment => {
                Pool::builder()
                    .build(
                        PostgresqlConnectionManager::new(
                            configuration.clone(),
                            NoTls,
                        ),
                    )
                    .await
                    .convert(Backtrace::new(line!(), file!()))?
            }
        };

        return Ok(postgresql_connection_pool);
    }
}
