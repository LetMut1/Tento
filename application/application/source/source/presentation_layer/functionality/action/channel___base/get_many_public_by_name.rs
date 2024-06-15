pub use crate::application_layer::functionality::action_processor::channel___base::get_many_public_by_name::Channel__Base___GetManyPublicByName;
use crate::application_layer::functionality::action_processor::ActionProcessor;
use crate::application_layer::functionality::service::processor::generalized_action::GeneralizedAction;
use crate::application_layer::functionality::service::processor::Processor;
use crate::infrastructure_layer::data::control_type::Response;
use crate::infrastructure_layer::data::environment_configuration::EnvironmentConfiguration;
#[cfg(feature = "manual_testing")]
use crate::infrastructure_layer::functionality::service::serializer::json::Json;
use crate::infrastructure_layer::functionality::service::serializer::message_pack::MessagePack;
use crate::presentation_layer::functionality::action::Action;
use crate::presentation_layer::functionality::service::extractor::http_body_data::HttpBodyData;
use crate::presentation_layer::functionality::service::extractor::Extractor;
use bb8::Pool;
use bb8_postgres::PostgresConnectionManager as PostgresqlConnectionManager;
use http::request::Parts;
use hyper::Body;
use matchit::Params;
use std::clone::Clone;
use std::marker::Send;
use std::marker::Sync;
use tokio_postgres::tls::MakeTlsConnect;
use tokio_postgres::tls::TlsConnect;
use tokio_postgres::Socket;
impl Action<Channel__Base___GetManyPublicByName> {
    pub async fn run<'a, T>(
        environment_configuration: &'a EnvironmentConfiguration,
        body: &'a mut Body,
        parts: &'a Parts,
        route_parameters: &'a Params<'_, '_>,
        database_1_postgresql_connection_pool: &'a Pool<PostgresqlConnectionManager<T>>,
        database_2_postgresql_connection_pool: &'a Pool<PostgresqlConnectionManager<T>>,
    ) -> Response
    where
        T: MakeTlsConnect<Socket> + Clone + Send + Sync + 'static,
        <T as MakeTlsConnect<Socket>>::Stream: Send + Sync,
        <T as MakeTlsConnect<Socket>>::TlsConnect: Send,
        <<T as MakeTlsConnect<Socket>>::TlsConnect as TlsConnect<Socket>>::Future: Send,
    {
        return Processor::<GeneralizedAction>::process::<'_, '_, '_, _, _, _, _, _, _, _, _, MessagePack>(
            environment_configuration,
            body,
            parts,
            route_parameters,
            database_1_postgresql_connection_pool,
            database_2_postgresql_connection_pool,
            Extractor::<HttpBodyData>::extract::<_, MessagePack>,
            ActionProcessor::<Channel__Base___GetManyPublicByName>::process,
        )
        .await;
    }
}
#[cfg(feature = "manual_testing")]
impl Action<Channel__Base___GetManyPublicByName> {
    pub async fn run_<'a, T>(
        environment_configuration: &'a EnvironmentConfiguration,
        body: &'a mut Body,
        parts: &'a Parts,
        route_parameters: &'a Params<'_, '_>,
        database_1_postgresql_connection_pool: &'a Pool<PostgresqlConnectionManager<T>>,
        database_2_postgresql_connection_pool: &'a Pool<PostgresqlConnectionManager<T>>,
    ) -> Response
    where
        T: MakeTlsConnect<Socket> + Clone + Send + Sync + 'static,
        <T as MakeTlsConnect<Socket>>::Stream: Send + Sync,
        <T as MakeTlsConnect<Socket>>::TlsConnect: Send,
        <<T as MakeTlsConnect<Socket>>::TlsConnect as TlsConnect<Socket>>::Future: Send,
    {
        return Processor::<GeneralizedAction>::process::<'_, '_, '_, _, _, _, _, _, _, _, _, Json>(
            environment_configuration,
            body,
            parts,
            route_parameters,
            database_1_postgresql_connection_pool,
            database_2_postgresql_connection_pool,
            Extractor::<HttpBodyData>::extract::<_, Json>,
            ActionProcessor::<Channel__Base___GetManyPublicByName>::process,
        )
        .await;
    }
}
