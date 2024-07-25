use super::Processor;
use crate::{
    application_layer::data::unified_report::UnifiedReport,
    infrastructure_layer::{
        data::{
            aggregate_error::{
                AggregateError,
                Backtrace,
            },
            control_type::{
                ActionRound,
                Response,
            },
            environment_configuration::EnvironmentConfiguration,
            server_workflow_error::ServerWorkflowError,
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
impl Processor<ActionRound> {
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
        F1: Future<Output = Result<I, AggregateError>>,
        AP: FnOnce(&'a EnvironmentConfiguration, &'a Pool<PostgresqlConnectionManager<T>>, &'a Pool<PostgresqlConnectionManager<T>>, I) -> F2,
        F2: Future<Output = Result<UnifiedReport<O, P>, AggregateError>>,
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
            action_processor,
        )
        .await
        {
            Ok(data) => {
                let response = Creator::<Response>::create_ok(data);
                Logger::<ActionRound>::log(
                    parts,
                    &response,
                );
                response
            }
            Err(aggregate_error) => {
                let response = match ServerWorkflowError::new(aggregate_error) {
                    ServerWorkflowError::Unexpected {
                        unexpected_auditor,
                    } => {
                        let response_ = Creator::<Response>::create_internal_server_error();
                        Logger::<ActionRound>::log_unexpected_auditor(
                            parts,
                            &response_,
                            unexpected_auditor,
                        );
                        response_
                    }
                    ServerWorkflowError::Expected {
                        expected_auditor,
                    } => {
                        let response_ = Creator::<Response>::create_bad_request();
                        Logger::<ActionRound>::log_expected_auditor(
                            parts,
                            &response_,
                            expected_auditor,
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
    ) -> Result<Vec<u8>, AggregateError>
    where
        T: MakeTlsConnect<Socket> + Clone + Send + Sync + 'static,
        <T as MakeTlsConnect<Socket>>::Stream: Send + Sync,
        <T as MakeTlsConnect<Socket>>::TlsConnect: Send,
        <<T as MakeTlsConnect<Socket>>::TlsConnect as TlsConnect<Socket>>::Future: Send,
        DE: FnOnce(&'a mut Body, &'a Parts, &'a Params<'b, 'c>) -> F1,
        F1: Future<Output = Result<I, AggregateError>>,
        AP: FnOnce(&'a EnvironmentConfiguration, &'a Pool<PostgresqlConnectionManager<T>>, &'a Pool<PostgresqlConnectionManager<T>>, I) -> F2,
        F2: Future<Output = Result<UnifiedReport<O, P>, AggregateError>>,
        O: SerdeSerialize,
        P: SerdeSerialize,
        Serializer<SF>: Serialize,
    {
        if !Validator::<Parts>::is_valid(parts) {
            return Err(
                AggregateError::new_invalid_argument_from_outside(
                    Backtrace::new(
                        line!(),
                        file!(),
                    ),
                ),
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
