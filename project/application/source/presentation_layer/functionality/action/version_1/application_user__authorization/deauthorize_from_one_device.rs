use crate::application_layer::functionality::action_processor::version_1::application_user__authorization::deauthorize_from_one_device::ActionProcessor;
use crate::application_layer::functionality::action_processor::version_1::application_user__authorization::deauthorize_from_one_device::Incoming;
use crate::application_layer::functionality::action_processor::version_1::application_user__authorization::deauthorize_from_one_device::Precedent;
use crate::application_layer::functionality::core_action_processor::CoreActionProcessor;
use crate::infrastructure_layer::data::control_type_registry::Request;
use crate::infrastructure_layer::data::control_type_registry::Response;
use crate::infrastructure_layer::data::environment_configuration::PushableEnvironmentConfiguration;
use crate::infrastructure_layer::data::void::Void;
use crate::infrastructure_layer::functionality::service::serializer::MessagePack;
use extern_crate::bb8_postgres::PostgresConnectionManager as PostgresqlConnectionManager;
use extern_crate::bb8_redis::RedisConnectionManager;
use extern_crate::bb8::Pool;
use extern_crate::tokio_postgres::Socket;
use extern_crate::tokio_postgres::tls::MakeTlsConnect;
use extern_crate::tokio_postgres::tls::TlsConnect;
use std::clone::Clone;
use std::marker::Send;
use std::marker::Sync;

#[cfg(feature = "facilitate_non_automatic_functional_testing")]
use crate::application_layer::functionality::service::wrapped_action_processor::WrappedActionProcessor;
#[cfg(feature = "facilitate_non_automatic_functional_testing")]
use crate::infrastructure_layer::functionality::service::serializer::Json;

pub struct DeauthorizeFromOneDevice;

impl DeauthorizeFromOneDevice {
    pub async fn run<'a, T>(
        pushable_environment_configuration: &'a PushableEnvironmentConfiguration,
        request: Request,
        database_1_postgresql_connection_pool: &'a Pool<PostgresqlConnectionManager<T>>,
        database_2_postgresql_connection_pool: &'a Pool<PostgresqlConnectionManager<T>>,
        database_1_redis_connection_pool: &'a Pool<RedisConnectionManager>
    ) -> Response
    where
        T: MakeTlsConnect<Socket> + Clone + Send + Sync + 'static,
        <T as MakeTlsConnect<Socket>>::Stream: Send + Sync,
        <T as MakeTlsConnect<Socket>>::TlsConnect: Send,
        <<T as MakeTlsConnect<Socket>>::TlsConnect as TlsConnect<Socket>>::Future: Send
    {
        return CoreActionProcessor::process::<'_, MessagePack, _, _, _, Incoming, Void, Precedent>(
            pushable_environment_configuration,
            request,
            database_1_postgresql_connection_pool,
            database_2_postgresql_connection_pool,
            database_1_redis_connection_pool,
            ActionProcessor::process
        ).await;
    }
}

#[cfg(feature = "facilitate_non_automatic_functional_testing")]
impl DeauthorizeFromOneDevice {
    pub async fn run_<'a, T>(
        pushable_environment_configuration: &'a PushableEnvironmentConfiguration,
        request: Request,
        database_1_postgresql_connection_pool: &'a Pool<PostgresqlConnectionManager<T>>,
        database_2_postgresql_connection_pool: &'a Pool<PostgresqlConnectionManager<T>>,
        database_1_redis_connection_pool: &'a Pool<RedisConnectionManager>
    ) -> Response
    where
        T: MakeTlsConnect<Socket> + Clone + Send + Sync + 'static,
        <T as MakeTlsConnect<Socket>>::Stream: Send + Sync,
        <T as MakeTlsConnect<Socket>>::TlsConnect: Send,
        <<T as MakeTlsConnect<Socket>>::TlsConnect as TlsConnect<Socket>>::Future: Send
    {
        return WrappedActionProcessor::process::<'_, Json, MessagePack, _, _, _, Incoming, Void, Precedent>(
            pushable_environment_configuration,
            request,
            database_1_postgresql_connection_pool,
            database_2_postgresql_connection_pool,
            database_1_redis_connection_pool,
            Self::run
        ).await;
    }
}