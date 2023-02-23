use crate::application_layer::functionality::service::action_round_result_writer::ActionRoundResultWriter;
use crate::infrastructure_layer::data::argument_result::InvalidArgument;
use crate::infrastructure_layer::data::environment_configuration::EnvironmentConfiguration;
use crate::infrastructure_layer::data::error_auditor::BacktracePart;
use crate::presentation_layer::functionality::service::action_response_creator::ActionResponseCreator;
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
    let response = ActionResponseCreator::create_not_found();

    if let Err(mut error) = ActionRoundResultWriter::write_with_context(
        database_2_postgresql_connection_pool, &request, &response, &InvalidArgument::HttpRoute
    ).await {
        error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

        unreachable!(
            "{}. TODO: Write in concurrent way. It is also necessary that the write
            process does not wait for another write process, and writes immediately.",
            &error
        );
    }

    return response;
}