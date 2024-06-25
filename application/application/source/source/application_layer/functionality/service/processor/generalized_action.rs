use super::Processor;
use crate::{
    application_layer::data::unified_report::UnifiedReport,
    infrastructure_layer::{
        data::{
            auditor::{
                Auditor,
                Backtrace,
            },
            control_type::{
                ActionRound,
                GeneralizedAction,
                Response,
            },
            environment_configuration::EnvironmentConfiguration,
            error::Error,
        },
        functionality::service::{
            creator::Creator,
            logger::Logger,
            serializer::{
                Serialize,
                Serializer,
            },
            validator::Validator,
        },
    },
};
use bb8::Pool;
use bb8_postgres::PostgresConnectionManager as PostgresqlConnectionManager;
use error::{External, Internal};
use http::request::Parts;
use hyper::Body;
use matchit::Params;
use serde::Serialize as SerdeSerialize;
use std::{
    clone::Clone,
    future::Future,
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
impl Processor<GeneralizedAction> {
    pub async fn process<'a, 'b, 'c, T, DE, F1, AP, F2, I, O, P, SF>(
        environment_configuration: &'a EnvironmentConfiguration,
        body: &'a mut Body,
        parts: &'a Parts,
        route_parameters: &'a Params<'b, 'c>,
        database_1_postgresql_connection_pool: &'a Pool<PostgresqlConnectionManager<T>>,
        database_2_postgresql_connection_pool: &'a Pool<PostgresqlConnectionManager<T>>,
        data_extractor: DE,
        action_processor: AP,
    ) -> Response
    where
        T: MakeTlsConnect<Socket> + Clone + Send + Sync + 'static,
        <T as MakeTlsConnect<Socket>>::Stream: Send + Sync,
        <T as MakeTlsConnect<Socket>>::TlsConnect: Send,
        <<T as MakeTlsConnect<Socket>>::TlsConnect as TlsConnect<Socket>>::Future: Send,
        DE: FnOnce(&'a mut Body, &'a Parts, &'a Params<'b, 'c>) -> F1,
        F1: Future<Output = Result<Option<I>, Error>>,
        AP: FnOnce(&'a EnvironmentConfiguration, &'a Pool<PostgresqlConnectionManager<T>>, &'a Pool<PostgresqlConnectionManager<T>>, Option<I>) -> F2,
        F2: Future<Output = Result<UnifiedReport<O, P>, Error>>,
        O: SerdeSerialize,
        P: SerdeSerialize,
        Serializer<SF>: Serialize,
    {
        return match Self::process_(
            environment_configuration,
            body,
            parts,
            route_parameters,
            database_1_postgresql_connection_pool,
            database_2_postgresql_connection_pool,
            data_extractor,
            action_processor
        )
        .await
        {
            Ok(data) => {
                let response = Creator::<Response>::create_ok(data);
                Logger::<(
                    ActionRound,
                    Response,
                )>::log(
                    parts,
                    &response,
                );
                response
            }
            Err(error) => {
                let response = match error {
                    Error::External { external_auditor } => {
                        let response_ = Creator::<Response>::create_bad_request();
                        Logger::<(
                            ActionRound,
                            Auditor<External>,
                        )>::log(
                            parts,
                            &response_,
                            external_auditor,
                        );
                        response_
                    }
                    Error::Internal { internal_auditor } => {
                        let response_ = Creator::<Response>::create_internal_server_error();
                        Logger::<(
                            ActionRound,
                            Auditor<Internal>,
                        )>::log(
                            parts,
                            &response_,
                            internal_auditor,
                        );
                        response_
                    }
                };
                response
            }
        };
    }

    async fn process_<'a, 'b, 'c, T, DE, F1, AP, F2, I, O, P, SF>(
        environment_configuration: &'a EnvironmentConfiguration,
        body: &'a mut Body,
        parts: &'a Parts,
        route_parameters: &'a Params<'b, 'c>,
        database_1_postgresql_connection_pool: &'a Pool<PostgresqlConnectionManager<T>>,
        database_2_postgresql_connection_pool: &'a Pool<PostgresqlConnectionManager<T>>,
        data_extractor: DE,
        action_processor: AP,
    ) -> Result<Vec<u8>, Error>
    where
        T: MakeTlsConnect<Socket> + Clone + Send + Sync + 'static,
        <T as MakeTlsConnect<Socket>>::Stream: Send + Sync,
        <T as MakeTlsConnect<Socket>>::TlsConnect: Send,
        <<T as MakeTlsConnect<Socket>>::TlsConnect as TlsConnect<Socket>>::Future: Send,
        DE: FnOnce(&'a mut Body, &'a Parts, &'a Params<'b, 'c>) -> F1,
        F1: Future<Output = Result<Option<I>, Error>>,
        AP: FnOnce(&'a EnvironmentConfiguration, &'a Pool<PostgresqlConnectionManager<T>>, &'a Pool<PostgresqlConnectionManager<T>>, Option<I>) -> F2,
        F2: Future<Output = Result<UnifiedReport<O, P>, Error>>,
        O: SerdeSerialize,
        P: SerdeSerialize,
        Serializer<SF>: Serialize
    {
        if !Validator::<Parts>::is_valid(parts) {
            return Err(
                Error::new_external_invalid_argument(
                    Backtrace::new(
                        line!(),
                        file!(),
                    ),
                )
            );
        }
        let incoming = data_extractor(
            body,
            parts,
            route_parameters,
        )
        .await?;
        let unified_report = action_processor(
            environment_configuration,
            database_1_postgresql_connection_pool,
            database_2_postgresql_connection_pool,
            incoming,
        )
        .await?;
        return Serializer::<SF>::serialize(&unified_report);
    }
}
