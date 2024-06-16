#[cfg(feature = "manual_testing")]
use crate::infrastructure_layer::data::control_type::Json;
use crate::{
    application_layer::functionality::{
        action_processor::ActionProcessor,
        service::processor::Processor,
    },
    infrastructure_layer::data::{
        control_type::{
            Channel__Base___GetManyByNameInSubscriptions,
            GeneralizedAction,
            HttpBodyData,
            MessagePack,
            Response,
        },
        environment_configuration::EnvironmentConfiguration,
    },
    presentation_layer::functionality::{
        action::Action,
        service::extractor::Extractor,
    },
};
use bb8::Pool;
use bb8_postgres::PostgresConnectionManager as PostgresqlConnectionManager;
use http::request::Parts;
use hyper::Body;
use matchit::Params;
use std::{
    clone::Clone,
    marker::{
        Send,
        Sync,
    },
};
use tokio_postgres::{
    tls::{
        MakeTlsConnect,
        TlsConnect,
    },
    Socket,
};
impl Action<Channel__Base___GetManyByNameInSubscriptions> {
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
            ActionProcessor::<Channel__Base___GetManyByNameInSubscriptions>::process,
        )
        .await;
    }
}
#[cfg(feature = "manual_testing")]
impl Action<Channel__Base___GetManyByNameInSubscriptions> {
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
            ActionProcessor::<Channel__Base___GetManyByNameInSubscriptions>::process,
        )
        .await;
    }
}
