use crate::application_layer::functionality::action_processor::application_user___authorization::register_by_last_step::Incoming;
use crate::application_layer::functionality::action_processor::application_user___authorization::register_by_last_step::RegisterByLastStep as RegisterByLastStep_;
use crate::application_layer::functionality::service::common_action_processor::CommonActionProcessor;
use crate::infrastructure_layer::data::control_type::Response;
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
use hyper::body::to_bytes;
use crate::infrastructure_layer::data::error_auditor::BacktracePart;
use crate::infrastructure_layer::data::error_auditor::Error;
use crate::infrastructure_layer::data::error_auditor::ErrorAuditor_;
use crate::infrastructure_layer::data::error_auditor::Other;
use crate::infrastructure_layer::data::error_auditor::Runtime;
use crate::infrastructure_layer::data::invalid_argument_result::InvalidArgumentResult;
use crate::infrastructure_layer::functionality::service::serializer::Serialize;
use crate::infrastructure_layer::functionality::service::serializer::Serializer;
use bytes::Buf;

#[cfg(feature = "manual_testing")]
use crate::infrastructure_layer::functionality::service::serializer::Json;

pub struct RegisterByLastStep;

impl RegisterByLastStep {
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
        return CommonActionProcessor::process::<'_, '_, '_, _, _, _, _, _, _, _, _, MessagePack>(
            body,
            parts,
            route_parameters,
            database_1_postgresql_connection_pool,
            database_2_postgresql_connection_pool,
            database_1_redis_connection_pool,
            Self::extract,
            RegisterByLastStep_::process,
        )
        .await;
    }

    pub async fn extract<'a>(
        body: &'a mut Body,
        _parts: &'a Parts,
        _route_parameters: &'a Params<'_, '_>,
    ) -> Result<InvalidArgumentResult<Option<Incoming>>, ErrorAuditor_> {
        let bytes = match to_bytes(body).await {
            Ok(bytes_) => bytes_,
            Err(error) => {
                return Err(
                    ErrorAuditor_::new(
                        Error::Runtime {
                            runtime: Runtime::Other {
                                other: Other::new(error),
                            },
                        },
                        BacktracePart::new(
                            line!(),
                            file!(),
                            None,
                        ),
                    )
                );
            }
        };

        let incoming = match Serializer::<MessagePack>::deserialize::<'_, Incoming>(bytes.chunk()) {
            Ok(incoming_) => incoming_,
            Err(mut error) => {
                error.add_backtrace_part(
                    BacktracePart::new(
                        line!(),
                        file!(),
                        None,
                    ),
                );

                return Err(error);
            }
        };

        return Ok(
            InvalidArgumentResult::Ok {
                subject: Some(incoming),
            },
        );
    }
}

#[cfg(feature = "manual_testing")]
impl RegisterByLastStep {
    pub async fn run_<'a, T>(
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
        return CommonActionProcessor::process::<'_, '_, '_, _, _, _, _, _, _, _, _, Json>(
            body,
            parts,
            route_parameters,
            database_1_postgresql_connection_pool,
            database_2_postgresql_connection_pool,
            database_1_redis_connection_pool,
            Self::extract,
            RegisterByLastStep_::process,
        )
        .await;
    }
}
