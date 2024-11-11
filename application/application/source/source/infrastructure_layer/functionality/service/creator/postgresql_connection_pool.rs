use super::Creator;
use crate::infrastructure_layer::data::{
    aggregate_error::{
        AggregateError,
        Backtrace,
        ResultConverter,
    },
    capture::Capture,
};
use deadpool_postgres::{
    Manager,
    ManagerConfig,
    RecyclingMethod
};
use tokio_postgres::{
    tls::{
        MakeTlsConnect,
        TlsConnect,
    },
    Socket,
};
use dedicated_crate::void::Void;
use std::{
    future::Future,
    str::FromStr,
};
use tokio_postgres::config::Config;
pub use deadpool_postgres::Pool as PostgresqlConnectionPool;
impl Creator<PostgresqlConnectionPool> {
    pub fn create<'a, T>(
        database_url: &'a str,
        tls_type: T,
    ) -> impl Future<Output = Result<PostgresqlConnectionPool, AggregateError>> + Send + Capture<&'a Void>
    where
        T: MakeTlsConnect<Socket> + Clone + Send + Sync + 'static,
        <T as MakeTlsConnect<Socket>>::Stream: Send + Sync,
        <T as MakeTlsConnect<Socket>>::TlsConnect: Send + Sync,
        <<T as MakeTlsConnect<Socket>>::TlsConnect as TlsConnect<Socket>>::Future: Send,
    {
        return async move {
            return PostgresqlConnectionPool::builder(
                Manager::from_config(
                    Config::from_str(database_url).into_logic(
                        Backtrace::new(
                            line!(),
                            file!(),
                        ),
                    )?,
                    tls_type,
                    ManagerConfig {
                        recycling_method: RecyclingMethod::Fast,
                    },
                )
            )
            .max_size(2)
            .build()
            .into_runtime(
                Backtrace::new(
                    line!(),
                    file!(),
                ),
            );
        };
    }
}
