use crate::application_layer::functionality::action_processor::version_1::application_user___authorization::authorize_by_last_step::ActionProcessor;
use crate::application_layer::functionality::action_processor::version_1::application_user___authorization::authorize_by_last_step::Incoming;
use crate::application_layer::functionality::action_processor::version_1::application_user___authorization::authorize_by_last_step::Outcoming;
use crate::application_layer::functionality::action_processor::version_1::application_user___authorization::authorize_by_last_step::Precedent;
use crate::application_layer::functionality::action_processor::core_action_processor::CoreActionProcessor;
use crate::infrastructure_layer::data::control_type::Request;
use crate::infrastructure_layer::data::control_type::Response;
use crate::infrastructure_layer::functionality::service::serializer::MessagePack;
use extern_crate::bb8::Pool;
use extern_crate::bb8_postgres::PostgresConnectionManager as PostgresqlConnectionManager;
use extern_crate::bb8_redis::RedisConnectionManager;
use extern_crate::tokio_postgres::tls::MakeTlsConnect;
use extern_crate::tokio_postgres::tls::TlsConnect;
use extern_crate::tokio_postgres::Socket;
use std::clone::Clone;
use std::marker::Send;
use std::marker::Sync;

#[cfg(feature = "manual_testing")]
use crate::application_layer::functionality::action_processor::wrapped_action_processor::WrappedActionProcessor;
#[cfg(feature = "manual_testing")]
use crate::infrastructure_layer::functionality::service::serializer::Json;

pub struct AuthorizeByLastStep;

impl AuthorizeByLastStep {
    pub async fn run<'a, T>(
        request: Request,
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
        return CoreActionProcessor::process::<'_, MessagePack, _, _, _, Incoming, Outcoming, Precedent>(
            request,
            database_1_postgresql_connection_pool,
            database_2_postgresql_connection_pool,
            database_1_redis_connection_pool,
            ActionProcessor::process,
        )
        .await;
    }
}

#[cfg(feature = "manual_testing")]
impl AuthorizeByLastStep {
    pub async fn run_<'a, T>(
        request: Request,
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
        return WrappedActionProcessor::process::<'_, Json, MessagePack, _, _, _, Incoming, Outcoming, Precedent>(
            request,
            database_1_postgresql_connection_pool,
            database_2_postgresql_connection_pool,
            database_1_redis_connection_pool,
            Self::run,
        )
        .await;
    }
}
