pub use deadpool_postgres::Pool as PostgresqlConnectionPool;
use {
    super::Creator,
    crate::infrastructure_layer::data::{
        aggregate_error::AggregateError,
        environment_configuration::PostgresqlInner,
    },
    deadpool::{
        Runtime,
        managed::QueueMode,
    },
    deadpool_postgres::{
        Manager,
        ManagerConfig,
        PoolConfig,
        RecyclingMethod,
        Timeouts,
    },
    std::{
        future::Future,
        time::Duration,
    },
    tokio_postgres::{
        Socket,
        tls::{
            MakeTlsConnect,
            TlsConnect,
        },
    },
};
impl Creator<PostgresqlConnectionPool> {
    pub fn create<'a, T>(postgresql_inner: &'a PostgresqlInner, tls_type: T) -> impl Future<Output = Result<PostgresqlConnectionPool, AggregateError>> + Send + use<'a, T>
    where
        T: MakeTlsConnect<Socket> + Clone + Send + Sync + 'static,
        <T as MakeTlsConnect<Socket>>::Stream: Send + Sync,
        <T as MakeTlsConnect<Socket>>::TlsConnect: Send + Sync,
        <<T as MakeTlsConnect<Socket>>::TlsConnect as TlsConnect<Socket>>::Future: Send,
    {
        return async move {
            return crate::result_into_runtime!(
                PostgresqlConnectionPool::builder(
                    Manager::from_config(
                        postgresql_inner.configuration.clone(),
                        tls_type,
                        ManagerConfig {
                            recycling_method: RecyclingMethod::Fast,
                        },
                    ),
                )
                .config(
                    PoolConfig {
                        max_size: postgresql_inner.maximum_connection_pool_size,
                        timeouts: Timeouts {
                            wait: Option::Some(Duration::from_millis(postgresql_inner.connection_pool_waiting_timeout_duration)),
                            create: Option::None,
                            recycle: Option::None,
                        },
                        queue_mode: QueueMode::Fifo,
                    }
                )
                .runtime(Runtime::Tokio1)
                .build()
            );
        };
    }
}
