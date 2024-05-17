use crate::application_layer::functionality::service::processor::Processor;
use crate::application_layer::functionality::service::processor::generalized_action::GeneralizedAction;
use crate::infrastructure_layer::data::control_type::Response;
use crate::infrastructure_layer::data::environment_configuration::EnvironmentConfiguration;
use crate::infrastructure_layer::functionality::service::serializer::message_pack::MessagePack;
use bb8::Pool;
use bb8_postgres::PostgresConnectionManager as PostgresqlConnectionManager;
use std::clone::Clone;
use std::marker::Send;
use std::marker::Sync;
use tokio_postgres::tls::MakeTlsConnect;
use tokio_postgres::tls::TlsConnect;
use tokio_postgres::Socket;
use http::request::Parts;
use hyper::Body;
use crate::presentation_layer::functionality::action::Action;
use matchit::Params;
use crate::presentation_layer::functionality::service::extractor::Extractor;
use crate::presentation_layer::functionality::service::extractor::http_body_data::HttpBodyData;
use crate::application_layer::functionality::action_processor::ActionProcessor;

pub use crate::application_layer::functionality::action_processor::application_user___authorization::authorize_by_first_step::ApplicationUser__Authorization___AuthorizeByFirstStep;

#[cfg(feature = "manual_testing")]
use crate::infrastructure_layer::functionality::service::serializer::json::Json;

impl Action<ApplicationUser__Authorization___AuthorizeByFirstStep> {
    pub async fn run<'a, T>(
        environment_configuration: &'static EnvironmentConfiguration,
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
            ActionProcessor::<ApplicationUser__Authorization___AuthorizeByFirstStep>::process,
        )
        .await;
    }
}

#[cfg(feature = "manual_testing")]
impl Action<ApplicationUser__Authorization___AuthorizeByFirstStep> {
    pub async fn run_<'a, T>(
        environment_configuration: &'static EnvironmentConfiguration,
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
            ActionProcessor::<ApplicationUser__Authorization___AuthorizeByFirstStep>::process,
        )
        .await;
    }
}
