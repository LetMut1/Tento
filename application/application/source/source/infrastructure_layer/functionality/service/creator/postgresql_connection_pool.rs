use super::Creator;
use crate::infrastructure_layer::data::{
    capture::Capture,
    control_type::PostgresqlConnectionPoolNoTls,
    environment_configuration::environment_configuration::EnvironmentConfiguration,
};
use aggregate_error::{
    AggregateError,
    Backtrace,
    ResultConverter,
};
use bb8::Pool;
use bb8_postgres::PostgresConnectionManager as PostgresqlConnectionManager;
use std::{
    clone::Clone,
    future::Future,
    str::FromStr,
};
use tokio_postgres::{
    config::Config,
    NoTls,
};
use void::Void;
impl Creator<PostgresqlConnectionPoolNoTls> {
    pub fn create_database_1<'a>(
        environment_configuration: &'a EnvironmentConfiguration,
    ) -> impl Future<Output = Result<PostgresqlConnectionPoolNoTls, AggregateError>> + Send + Capture<&'a Void> {
        return async move {
            return Self::create(
                &Config::from_str(environment_configuration.resource.postgresql.database_1_url.as_str()).into_logic(
                    Backtrace::new(
                        line!(),
                        file!(),
                    ),
                )?,
            )
            .await;
        };
    }
    pub fn create_database_2<'a>(
        environment_configuration: &'a EnvironmentConfiguration,
    ) -> impl Future<Output = Result<PostgresqlConnectionPoolNoTls, AggregateError>> + Send + Capture<&'a Void> {
        return async move {
            return Self::create(
                &Config::from_str(environment_configuration.resource.postgresql.database_2_url.as_str()).into_logic(
                    Backtrace::new(
                        line!(),
                        file!(),
                    ),
                )?,
            )
            .await;
        };
    }
    fn create<'a>(configuration: &'a Config) -> impl Future<Output = Result<PostgresqlConnectionPoolNoTls, AggregateError>> + Send + Capture<&'a Void> {
        return async move {
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
        };
    }
}
