use crate::application_layer::functionality::action_processor::route_not_found::RouteNotFound as RouteNotFound_;
use crate::infrastructure_layer::data::control_type::Request;
use crate::infrastructure_layer::data::control_type::Response;
use extern_crate::bb8::Pool;
use extern_crate::bb8_postgres::PostgresConnectionManager as PostgresqlConnectionManager;
use extern_crate::tokio_postgres::tls::MakeTlsConnect;
use extern_crate::tokio_postgres::tls::TlsConnect;
use extern_crate::tokio_postgres::Socket;
use std::clone::Clone;
use std::marker::Send;
use std::marker::Sync;

pub struct RouteNotFound;

impl RouteNotFound {
    pub async fn run<'a, T>(
        request: Request,
        database_2_postgresql_connection_pool: &'a Pool<PostgresqlConnectionManager<T>>,
    ) -> Response
    where
        T: MakeTlsConnect<Socket> + Clone + Send + Sync + 'static,
        <T as MakeTlsConnect<Socket>>::Stream: Send + Sync,
        <T as MakeTlsConnect<Socket>>::TlsConnect: Send,
        <<T as MakeTlsConnect<Socket>>::TlsConnect as TlsConnect<Socket>>::Future: Send,
    {
        return RouteNotFound_::process(
            request,
            database_2_postgresql_connection_pool,
        )
        .await;
    }
}