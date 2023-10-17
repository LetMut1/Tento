use crate::application_layer::functionality::action_processor::health_check::HealthCheck as HealthCheck_;
use crate::application_layer::functionality::service::processor::Processor;
use crate::application_layer::functionality::service::processor::Action;
use crate::infrastructure_layer::data::control_type::Response;
use crate::infrastructure_layer::data::void::Void;
use crate::infrastructure_layer::functionality::service::serializer::MessagePack;
use bb8::Pool;
use bb8_postgres::PostgresConnectionManager as PostgresqlConnectionManager;
use bb8_redis::RedisConnectionManager;
use std::clone::Clone;
use std::marker::Send;
use std::marker::Sync;
use tokio_postgres::tls::MakeTlsConnect;
use tokio_postgres::tls::TlsConnect;
use tokio_postgres::Socket;
use http::request::Parts;
use hyper::Body;
use matchit::Params;
use crate::application_layer::functionality::action_processor::action_processor::ActionProcessor;
use crate::infrastructure_layer::data::error_auditor::ErrorAuditor_;
use crate::infrastructure_layer::data::invalid_argument_result::InvalidArgumentResult;


pub struct HealthCheck;

impl HealthCheck {
    pub async fn run<'a, T>(
        body: &'a mut Body,
        parts: &'a Parts,
        route_parameters: &'a Params<'_, '_>,
        database_1_postgresql_connection_pool: &'a Pool<PostgresqlConnectionManager<T>>,
        database_2_postgresql_connection_pool: &'a Pool<PostgresqlConnectionManager<T>>,
        database_1_redis_connection_pool: &'a Pool<RedisConnectionManager>,
    ) -> Response
    where
        T: MakeTlsConnect<Socket> + Clone + Send + Sync + 'static,
        <T as MakeTlsConnect<Socket>>::Stream: Send + Sync,
        <T as MakeTlsConnect<Socket>>::TlsConnect: Send,
        <<T as MakeTlsConnect<Socket>>::TlsConnect as TlsConnect<Socket>>::Future: Send,
    {
        return Processor::<Action>::process::<'_, '_, '_, _, _, _, _, _, _, _, _, MessagePack>(
            body,
            parts,
            route_parameters,
            database_1_postgresql_connection_pool,
            database_2_postgresql_connection_pool,
            database_1_redis_connection_pool,
            Self::extract,
            ActionProcessor::<HealthCheck_>::process,
        )
        .await;
    }

    pub async fn extract<'a>(
        _body: &'a mut Body,
        _parts: &'a Parts,
        _route_parameters: &'a Params<'_, '_>,
    ) -> Result<InvalidArgumentResult<Option<Void>>, ErrorAuditor_> {
        return Ok(
            InvalidArgumentResult::Ok {
                subject: None
            },
        );
    }
}