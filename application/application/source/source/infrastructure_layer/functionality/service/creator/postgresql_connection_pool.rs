use super::Creator;
use crate::infrastructure_layer::data::environment_configuration::Environment;
use crate::infrastructure_layer::data::auditor::BacktracePart;
use crate::infrastructure_layer::data::error::Error;
use crate::infrastructure_layer::data::auditor::Auditor;
use crate::infrastructure_layer::data::auditor::Converter;
use bb8::Pool;
use bb8_postgres::PostgresConnectionManager as PostgresqlConnectionManager;
use std::clone::Clone;
use tokio_postgres::config::Config;
use tokio_postgres::NoTls;

pub use crate::infrastructure_layer::data::control_type::PostgresqlConnectionPoolNoTls;

impl Creator<PostgresqlConnectionPoolNoTls> {
    pub async fn create<'a>(
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
                        BacktracePart::new(
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
                    .convert(BacktracePart::new(line!(), file!()))?
            }
        };

        return Ok(postgresql_connection_pool);
    }
}
