use crate::infrastructure_layer::data::environment_configuration::EnvironmentConfiguration;
use crate::application_layer::functionality::service::action_processor::route_not_found::ActionProcessor;
use extern_crate::bb8_postgres::PostgresConnectionManager as PostgresqlConnectionManager;
use extern_crate::bb8_redis::RedisConnectionManager;
use extern_crate::bb8::Pool;
use extern_crate::hyper::Body;
use extern_crate::hyper::Request;
use extern_crate::hyper::Response;
use extern_crate::tokio_postgres::Socket;
use extern_crate::tokio_postgres::tls::MakeTlsConnect;
use extern_crate::tokio_postgres::tls::TlsConnect;
use std::clone::Clone;
use std::marker::Send;
use std::marker::Sync;

pub async fn route_not_found<'a, T>(
    _environment_configuration: &'a EnvironmentConfiguration,
    request: Request<Body>,
    _database_1_postgresql_connection_pool: &'a Pool<PostgresqlConnectionManager<T>>,
    database_2_postgresql_connection_pool: &'a Pool<PostgresqlConnectionManager<T>>,
    _redis_connection_pool: &'a Pool<RedisConnectionManager>
) -> Response<Body>
where
    T: MakeTlsConnect<Socket> + Clone + Send + Sync + 'static,
    <T as MakeTlsConnect<Socket>>::Stream: Send + Sync,
    <T as MakeTlsConnect<Socket>>::TlsConnect: Send,
    <<T as MakeTlsConnect<Socket>>::TlsConnect as TlsConnect<Socket>>::Future: Send
{
    return ActionProcessor::process(request, database_2_postgresql_connection_pool).await;
}