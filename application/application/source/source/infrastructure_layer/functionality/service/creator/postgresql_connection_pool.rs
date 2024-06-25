use super::Creator;
use crate::infrastructure_layer::data::{
    auditor::{
        Backtrace,
    },
    control_type::PostgresqlConnectionPoolNoTls,
    environment_configuration::EnvironmentConfiguration,
    alternative_workflow::AlternativeWorkflow,
    alternative_workflow::ResultConverter,
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
    pub async fn create_database_1<'a>(environment_configuration: &'a EnvironmentConfiguration) -> Result<Pool<PostgresqlConnectionManager<NoTls>>, AlternativeWorkflow> {
        return Self::create(
            &Config::from_str(environment_configuration.resource.postgresql.database_1_url.as_str()).convert_into_error(
                Backtrace::new(
                    line!(),
                    file!(),
                ),
            )?,
        )
        .await;
    }
    pub async fn create_database_2<'a>(environment_configuration: &'a EnvironmentConfiguration) -> Result<Pool<PostgresqlConnectionManager<NoTls>>, AlternativeWorkflow> {
        return Self::create(
            &Config::from_str(environment_configuration.resource.postgresql.database_2_url.as_str()).convert_into_error(
                Backtrace::new(
                    line!(),
                    file!(),
                ),
            )?,
        )
        .await;
    }
    async fn create<'a>(configuration: &'a Config) -> Result<Pool<PostgresqlConnectionManager<NoTls>>, AlternativeWorkflow> {
        return Pool::builder()
            .build(
                PostgresqlConnectionManager::new(
                    configuration.clone(),
                    NoTls,
                ),
            )
            .await
            .convert_into_error(
                Backtrace::new(
                    line!(),
                    file!(),
                ),
            );
    }
}
