use super::Creator;
use crate::infrastructure_layer::data::{
    aggregate_error::{
        AggregateError,
        Backtrace,
        ResultConverter,
    },
    control_type::PostgresqlConnectionPoolNoTls,
    environment_configuration::EnvironmentConfiguration,
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
    pub async fn create_database_1<'a>(environment_configuration: &'a EnvironmentConfiguration) -> Result<Pool<PostgresqlConnectionManager<NoTls>>, AggregateError> {
        return Self::create(
            &Config::from_str(environment_configuration.resource.postgresql.database_1_url.as_str()).into_invalid_argument_from_client_code(
                Backtrace::new(
                    line!(),
                    file!(),
                ),
            )?,
        )
        .await;
    }
    pub async fn create_database_2<'a>(environment_configuration: &'a EnvironmentConfiguration) -> Result<Pool<PostgresqlConnectionManager<NoTls>>, AggregateError> {
        return Self::create(
            &Config::from_str(environment_configuration.resource.postgresql.database_2_url.as_str()).into_invalid_argument_from_client_code(
                Backtrace::new(
                    line!(),
                    file!(),
                ),
            )?,
        )
        .await;
    }
    async fn create<'a>(configuration: &'a Config) -> Result<Pool<PostgresqlConnectionManager<NoTls>>, AggregateError> {
        return Pool::builder()
            .build(
                PostgresqlConnectionManager::new(
                    configuration.clone(),
                    NoTls,
                ),
            )
            .await
            .into_runtime(
                Backtrace::new(
                    line!(),
                    file!(),
                ),
            );
    }
}
