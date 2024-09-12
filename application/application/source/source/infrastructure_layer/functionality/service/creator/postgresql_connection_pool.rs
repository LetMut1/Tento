use super::Creator;
use crate::infrastructure_layer::data::capture::Capture;
use aggregate_error::{
    AggregateError,
    Backtrace,
    ResultConverter,
};
use bb8::Pool;
use bb8_postgres::PostgresConnectionManager;
use std::{
    future::Future,
    str::FromStr,
};
use tokio_postgres::{
    config::Config,
    NoTls,
};
use void::Void;
impl Creator<Pool<PostgresConnectionManager<NoTls>>> {
    pub fn create<'a>(database_url: &'a str) -> impl Future<Output = Result<Pool<PostgresConnectionManager<NoTls>>, AggregateError>> + Send + Capture<&'a Void> {
        return async move {
            return Pool::builder()
                .build(
                    PostgresConnectionManager::new(
                        Config::from_str(database_url).into_logic(
                            Backtrace::new(
                                line!(),
                                file!(),
                            ),
                        )?,
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
