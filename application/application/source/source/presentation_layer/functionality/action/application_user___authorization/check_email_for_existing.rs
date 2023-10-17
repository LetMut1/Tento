use crate::application_layer::functionality::service::processor::Processor;
use crate::application_layer::functionality::service::action___processor::GeneralizedAction;
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
use crate::presentation_layer::functionality::action::Action;
use hyper::Body;
use matchit::Params;
use crate::application_layer::functionality::action_processor::ActionProcessor;
use crate::presentation_layer::functionality::service::extractor::Extractor;
use crate::presentation_layer::functionality::service::extractor::HttpBodyData;

pub use crate::application_layer::functionality::action_processor::application_user___authorization::check_email_for_existing::ApplicationUser__Authorization___CheckEmailForExisting;

#[cfg(feature = "manual_testing")]
use crate::infrastructure_layer::functionality::service::serializer::Json;

impl Action<ApplicationUser__Authorization___CheckEmailForExisting> {
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
        return Processor::<GeneralizedAction>::process::<'_, '_, '_, _, _, _, _, _, _, _, _, MessagePack>(
            body,
            parts,
            route_parameters,
            database_1_postgresql_connection_pool,
            database_2_postgresql_connection_pool,
            database_1_redis_connection_pool,
            Extractor::<HttpBodyData>::extract::<_, MessagePack>,
            ActionProcessor::<ApplicationUser__Authorization___CheckEmailForExisting>::process,
        )
        .await;
    }
}

#[cfg(feature = "manual_testing")]
impl Action<ApplicationUser__Authorization___CheckEmailForExisting> {
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
        return Processor::<GeneralizedAction>::process::<'_, '_, '_, _, _, _, _, _, _, _, _, Json>(
            body,
            parts,
            route_parameters,
            database_1_postgresql_connection_pool,
            database_2_postgresql_connection_pool,
            database_1_redis_connection_pool,
            Extractor::<HttpBodyData>::extract::<_, Json>,
            ActionProcessor::<ApplicationUser__Authorization___CheckEmailForExisting>::process,
        )
        .await;
    }
}
