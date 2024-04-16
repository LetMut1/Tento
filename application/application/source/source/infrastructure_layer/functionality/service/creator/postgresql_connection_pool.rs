use super::Creator;
use crate::infrastructure_layer::data::environment_configuration::Environment;
use crate::infrastructure_layer::data::error_auditor::BacktracePart;
use crate::infrastructure_layer::data::error_auditor::Error;
use crate::infrastructure_layer::data::error_auditor::Auditor;
use crate::infrastructure_layer::data::error_auditor::Runtime;
use crate::infrastructure_layer::data::error_auditor::Other;
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
                let postgresql_connection_pool_ = match Pool::builder()
                    .build(
                        PostgresqlConnectionManager::new(
                            configuration.clone(),
                            NoTls,
                        ),
                    )
                    .await
                {
                    Ok(postgresql_connection_pool__) => postgresql_connection_pool__,
                    Err(error) => {
                        return Err(
                            Auditor::<Error>::new(
                                Error::Runtime {
                                    runtime: Runtime::Other {
                                        other: Other::new(error),
                                    },
                                },
                                BacktracePart::new(
                                    line!(),
                                    file!(),
                                ),
                            ),
                        );
                    }
                };

                postgresql_connection_pool_
            }
        };

        return Ok(postgresql_connection_pool);
    }
}
