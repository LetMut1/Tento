use crate::infrastructure_layer::data::control_type::Response;
use bb8::Pool;
use bb8_postgres::PostgresConnectionManager as PostgresqlConnectionManager;
use std::clone::Clone;
use std::marker::Send;
use std::marker::Sync;
use tokio_postgres::tls::MakeTlsConnect;
use tokio_postgres::tls::TlsConnect;
use tokio_postgres::Socket;
use crate::presentation_layer::functionality::action::Action;
use http::request::Parts;
use crate::application_layer::functionality::action_processor::ActionProcessor;

pub use crate::application_layer::functionality::action_processor::route_not_found::RouteNotFound;

impl Action<RouteNotFound> {
    pub async fn run<'a, T>(
        parts: &'a Parts,
        database_2_postgresql_connection_pool: &'a Pool<PostgresqlConnectionManager<T>>,
    ) -> Response
    where
        T: MakeTlsConnect<Socket> + Clone + Send + Sync + 'static,
        <T as MakeTlsConnect<Socket>>::Stream: Send + Sync,
        <T as MakeTlsConnect<Socket>>::TlsConnect: Send,
        <<T as MakeTlsConnect<Socket>>::TlsConnect as TlsConnect<Socket>>::Future: Send,
    {
        return ActionProcessor::<RouteNotFound>::process(
            parts,
            database_2_postgresql_connection_pool,
        )
        .await;
    }
}
