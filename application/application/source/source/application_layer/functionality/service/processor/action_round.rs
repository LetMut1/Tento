use super::Processor;
use crate::{
    application_layer::data::unified_report::UnifiedReport,
    infrastructure_layer::{
        data::{
            aggregate_error::{
                AggregateError,
                Backtrace,
                ResultConverter,
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
use bytes::Buf;
use bb8::Pool;
use bb8_postgres::PostgresConnectionManager as PostgresqlConnectionManager;
use http::request::Parts;
use hyper::{
    body::to_bytes,
    Body,
};
use matchit::Params;
use serde::{
    Serialize as SerdeSerialize,
    Deserialize as SerdeDeserialize,
};
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
    pub async fn process<'a, 'b, 'c, T, AP, F, I, O, P, SS, SD>(
        environment_configuration: &'a EnvironmentConfiguration,
        body: &'a mut Body,
        parts: &'a Parts,
        route_parameters: &'a Params<'b, 'c>,
        database_1_postgresql_connection_pool: &'a Pool<PostgresqlConnectionManager<T>>,
        database_2_postgresql_connection_pool: &'a Pool<PostgresqlConnectionManager<T>>,
        action_processor: AP,
    ) -> Response
    where
        T: MakeTlsConnect<Socket> + Clone + Send + Sync + 'static,
        <T as MakeTlsConnect<Socket>>::Stream: Send + Sync,
        <T as MakeTlsConnect<Socket>>::TlsConnect: Send,
        <<T as MakeTlsConnect<Socket>>::TlsConnect as TlsConnect<Socket>>::Future: Send,
        AP: FnOnce(&'a EnvironmentConfiguration, &'a Pool<PostgresqlConnectionManager<T>>, &'a Pool<PostgresqlConnectionManager<T>>, I) -> F,
        F: Future<Output = Result<UnifiedReport<O, P>, AggregateError>>,
        I: for<'de> SerdeDeserialize<'de>,
        O: SerdeSerialize,
        P: SerdeSerialize,
        Serializer<SS>: Serialize,
        Serializer<SD>: Serialize,
        'b: 'b,
        'c: 'c,
    {
        return match Self::process_::<'_, '_, '_, _, _, _, _, _, _, SS, SD>(
            environment_configuration,
            body,
            parts,
            route_parameters,
            database_1_postgresql_connection_pool,
            database_2_postgresql_connection_pool,
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
    async fn process_<'a, 'b, 'c, T, AP, F2, I, O, P, SS, SD>(
        environment_configuration: &'a EnvironmentConfiguration,
        body: &'a mut Body,
        parts: &'a Parts,
        route_parameters: &'a Params<'b, 'c>,
        database_1_postgresql_connection_pool: &'a Pool<PostgresqlConnectionManager<T>>,
        database_2_postgresql_connection_pool: &'a Pool<PostgresqlConnectionManager<T>>,
        action_processor: AP,
    ) -> Result<Vec<u8>, AggregateError>
    where
        T: MakeTlsConnect<Socket> + Clone + Send + Sync + 'static,
        <T as MakeTlsConnect<Socket>>::Stream: Send + Sync,
        <T as MakeTlsConnect<Socket>>::TlsConnect: Send,
        <<T as MakeTlsConnect<Socket>>::TlsConnect as TlsConnect<Socket>>::Future: Send,
        AP: FnOnce(&'a EnvironmentConfiguration, &'a Pool<PostgresqlConnectionManager<T>>, &'a Pool<PostgresqlConnectionManager<T>>, I) -> F2,
        F2: Future<Output = Result<UnifiedReport<O, P>, AggregateError>>,
        I: for<'de> SerdeDeserialize<'de>,
        O: SerdeSerialize,
        P: SerdeSerialize,
        Serializer<SS>: Serialize,
        Serializer<SD>: Serialize,
        'b: 'b,
        'c: 'c,
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

        let bytes = to_bytes(body).await.into_invalid_argument_from_outside_and_client_code(
            Backtrace::new(
                line!(),
                file!(),
            ),
        )?;
        let incoming = Serializer::<SS>::deserialize::<'_, I>(bytes.chunk())?;

        let unified_report = action_processor(
            environment_configuration,
            database_1_postgresql_connection_pool,
            database_2_postgresql_connection_pool,
            incoming,
        )
        .await?;
        return Serializer::<SD>::serialize(&unified_report);
    }
}
