pub mod application_user___authorization;
pub mod channel___base;
pub mod channel_subscription___base;
use crate::infrastructure_layer::data::{
    capture::Capture,
    environment_configuration::environment_configuration::EnvironmentConfiguration,
};
use aggregate_error::{
    AggregateError,
    Backtrace,
    ResultConverter,
};
use bb8::{
    Pool,
    PooledConnection,
};
use bb8_postgres::PostgresConnectionManager;
use std::{
    future::Future,
    marker::PhantomData,
};
use tokio_postgres::{
    tls::{
        MakeTlsConnect,
        TlsConnect,
    },
    Socket,
};
use unified_report::UnifiedReport;
use void::Void;
pub struct ActionProcessor<S> {
    _subject: PhantomData<S>,
}
pub trait ActionProcessor_ {
    type Incoming;
    type Outcoming;
    type Precedent;
    fn process<'a, T>(
        inner: &'a Inner<'_, T>,
        incoming: Self::Incoming,
    ) -> impl Future<Output = Result<UnifiedReport<Self::Outcoming, Self::Precedent>, AggregateError>> + Send + Capture<&'a Void>
    where
        T: MakeTlsConnect<Socket> + Clone + Send + Sync + 'static,
        <T as MakeTlsConnect<Socket>>::Stream: Send + Sync,
        <T as MakeTlsConnect<Socket>>::TlsConnect: Send,
        <<T as MakeTlsConnect<Socket>>::TlsConnect as TlsConnect<Socket>>::Future: Send;
}
pub struct Inner<'a, T>
where
    T: MakeTlsConnect<Socket> + Clone + Send + Sync + 'static,
    <T as MakeTlsConnect<Socket>>::Stream: Send + Sync,
    <T as MakeTlsConnect<Socket>>::TlsConnect: Send,
    <<T as MakeTlsConnect<Socket>>::TlsConnect as TlsConnect<Socket>>::Future: Send,
{
    pub environment_configuration: &'a EnvironmentConfiguration,
    pub database_1_postgresql_connection_pool: &'a Pool<PostgresConnectionManager<T>>,
    pub database_2_postgresql_connection_pool: &'a Pool<PostgresConnectionManager<T>>,
}
impl<'a, T> Inner<'a, T>
where
    T: MakeTlsConnect<Socket> + Clone + Send + Sync + 'static,
    <T as MakeTlsConnect<Socket>>::Stream: Send + Sync,
    <T as MakeTlsConnect<Socket>>::TlsConnect: Send,
    <<T as MakeTlsConnect<Socket>>::TlsConnect as TlsConnect<Socket>>::Future: Send,
{
    pub fn get_database_1_postgresql_pooled_connection<'b>(&'b self) -> impl Future<Output = Result<PooledConnection<PostgresConnectionManager<T>>, AggregateError>> + Send {
        return async move {
            return self.database_1_postgresql_connection_pool.get().await.into_runtime(
                Backtrace::new(
                    line!(),
                    file!(),
                ),
            );
        };
    }
    pub fn get_database_2_postgresql_pooled_connection<'b>(&'b self) -> impl Future<Output = Result<PooledConnection<PostgresConnectionManager<T>>, AggregateError>> + Send {
        return async move {
            return self.database_2_postgresql_connection_pool.get().await.into_runtime(
                Backtrace::new(
                    line!(),
                    file!(),
                ),
            );
        };
    }
}
