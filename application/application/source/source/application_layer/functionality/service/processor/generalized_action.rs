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
            invalid_argument::InvalidArgument,
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
        F1: Future<Output = Result<Result<Option<I>, Auditor<InvalidArgument>>, Auditor<Error>>>,
        AP: FnOnce(&'a EnvironmentConfiguration, &'a Pool<PostgresqlConnectionManager<T>>, &'a Pool<PostgresqlConnectionManager<T>>, Option<I>) -> F2,
        F2: Future<Output = Result<Result<UnifiedReport<O, P>, Auditor<InvalidArgument>>, Auditor<Error>>>,
        O: SerdeSerialize,
        P: SerdeSerialize,
        Serializer<SF>: Serialize,
    {
        if !Validator::<Parts>::is_valid(parts) {
            let response = Creator::<Response>::create_bad_request();
            Logger::<(
                ActionRound,
                Auditor<InvalidArgument>,
            )>::log(
                parts,
                &response,
                Auditor::<InvalidArgument>::new(
                    InvalidArgument,
                    Backtrace::new(
                        line!(),
                        file!(),
                    ),
                ),
            );
            return response;
        }
        let incoming = match data_extractor(
            body,
            parts,
            route_parameters,
        )
        .await
        {
            Ok(incoming_) => incoming_,
            Err(error_auditor) => {
                let response = Creator::<Response>::create_internal_server_error();
                Logger::<(
                    ActionRound,
                    Auditor<Error>,
                )>::log(
                    parts,
                    &response,
                    error_auditor,
                );
                return response;
            }
        };
        let incoming_ = match incoming {
            Ok(incoming__) => incoming__,
            Err(invalid_argument_auditor) => {
                let response = Creator::<Response>::create_bad_request();
                Logger::<(
                    ActionRound,
                    Auditor<InvalidArgument>,
                )>::log(
                    parts,
                    &response,
                    invalid_argument_auditor,
                );
                return response;
            }
        };
        let unified_report = match action_processor(
            environment_configuration,
            database_1_postgresql_connection_pool,
            database_2_postgresql_connection_pool,
            incoming_,
        )
        .await
        {
            Ok(unified_report_) => unified_report_,
            Err(error_auditor) => {
                let response = Creator::<Response>::create_internal_server_error();
                Logger::<(
                    ActionRound,
                    Auditor<Error>,
                )>::log(
                    parts,
                    &response,
                    error_auditor,
                );
                return response;
            }
        };
        let unified_report_ = match unified_report {
            Ok(unified_report__) => unified_report__,
            Err(invalid_argument_auditor) => {
                let response = Creator::<Response>::create_bad_request();
                Logger::<(
                    ActionRound,
                    Auditor<InvalidArgument>,
                )>::log(
                    parts,
                    &response,
                    invalid_argument_auditor,
                );
                return response;
            }
        };
        let data = match Serializer::<SF>::serialize(&unified_report_) {
            Ok(data_) => data_,
            Err(error_auditor) => {
                let response = Creator::<Response>::create_internal_server_error();
                Logger::<(
                    ActionRound,
                    Auditor<Error>,
                )>::log(
                    parts,
                    &response,
                    error_auditor,
                );
                return response;
            }
        };
        let response = Creator::<Response>::create_ok(data);
        Logger::<(
            ActionRound,
            Response,
        )>::log(
            parts,
            &response,
        );
        return response;
    }
}
