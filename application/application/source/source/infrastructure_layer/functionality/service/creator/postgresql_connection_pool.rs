use super::Creator;
use crate::infrastructure_layer::data::auditor::Auditor;
use crate::infrastructure_layer::data::auditor::Backtrace;
use crate::infrastructure_layer::data::auditor::ErrorConverter;
pub use crate::infrastructure_layer::data::control_type::PostgresqlConnectionPoolNoTls;
use crate::infrastructure_layer::data::environment_configuration::EnvironmentConfiguration;
use crate::infrastructure_layer::data::error::Error;
use bb8::Pool;
use bb8_postgres::PostgresConnectionManager as PostgresqlConnectionManager;
use std::clone::Clone;
use std::str::FromStr;
use tokio_postgres::config::Config;
use tokio_postgres::NoTls;
impl Creator<PostgresqlConnectionPoolNoTls> {
    pub async fn create_database_1<'a>(environment_configuration: &'a EnvironmentConfiguration) -> Result<Pool<PostgresqlConnectionManager<NoTls>>, Auditor<Error>> {
        return Ok(Self::create(&Config::from_str(environment_configuration.resource.postgresql.database_1_url.as_str()).convert(Backtrace::new(line!(), file!()))?).await?);
    }
    pub async fn create_database_2<'a>(environment_configuration: &'a EnvironmentConfiguration) -> Result<Pool<PostgresqlConnectionManager<NoTls>>, Auditor<Error>> {
        return Ok(Self::create(&Config::from_str(environment_configuration.resource.postgresql.database_2_url.as_str()).convert(Backtrace::new(line!(), file!()))?).await?);
    }
    async fn create<'a>(configuration: &'a Config) -> Result<Pool<PostgresqlConnectionManager<NoTls>>, Auditor<Error>> {
        return Ok(Pool::builder()
            .build(PostgresqlConnectionManager::new(
                configuration.clone(),
                NoTls,
            ))
            .await
            .convert(Backtrace::new(line!(), file!()))?);
    }
}
