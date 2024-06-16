use super::Creator;
pub use crate::infrastructure_layer::data::control_type::PostgresqlConnectionPoolNoTls;
use crate::infrastructure_layer::data::{
    auditor::{
        Auditor,
        Backtrace,
        ErrorConverter,
    },
    environment_configuration::EnvironmentConfiguration,
    error::Error,
};
use bb8::Pool;
use bb8_postgres::PostgresConnectionManager as PostgresqlConnectionManager;
use std::{
    clone::Clone,
    str::FromStr,
};
use tokio_postgres::{
    config::Config,
    NoTls,
};
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
