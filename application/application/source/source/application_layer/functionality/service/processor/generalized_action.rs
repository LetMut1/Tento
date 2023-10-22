use crate::application_layer::data::unified_report::UnifiedReport;
use crate::infrastructure_layer::data::error_auditor::ErrorAuditor;
use crate::infrastructure_layer::data::invalid_argument_result::InvalidArgument;
use crate::infrastructure_layer::data::invalid_argument_result::InvalidArgumentResult;
use crate::infrastructure_layer::functionality::service::creator::Creator;
use crate::infrastructure_layer::functionality::service::creator::response::Response;
use crate::infrastructure_layer::functionality::service::serializer::Serialize;
use crate::infrastructure_layer::functionality::service::serializer::Serializer;
use crate::infrastructure_layer::functionality::service::validator::Validator;
use bb8::Pool;
use bb8_postgres::PostgresConnectionManager as PostgresqlConnectionManager;
use bb8_redis::RedisConnectionManager;
use serde::Serialize as SerdeSerialize;
use std::clone::Clone;
use std::future::Future;
use crate::application_layer::functionality::service::reactor::Reactor;
use std::marker::Send;
use std::marker::Sync;
use tokio_postgres::tls::MakeTlsConnect;
use tokio_postgres::tls::TlsConnect;
use tokio_postgres::Socket;
use http::request::Parts;
use hyper::Body;
use super::Processor;
use matchit::Params;

pub use crate::infrastructure_layer::data::control_type::GeneralizedAction;

impl Processor<GeneralizedAction> {
    pub async fn process<'a, 'b, 'c, T, DE, F1, AP, F2, I, O, P, SF>(
        body: &'a mut Body,
        parts: &'a Parts,
        route_parameters: &'a Params<'b, 'c>,
        database_1_postgresql_connection_pool: &'a Pool<PostgresqlConnectionManager<T>>,
        database_2_postgresql_connection_pool: &'a Pool<PostgresqlConnectionManager<T>>,
        database_1_redis_connection_pool: &'a Pool<RedisConnectionManager>,
        data_extractor: DE,
        action_processor: AP,
    ) -> Response
    where
        T: MakeTlsConnect<Socket> + Clone + Send + Sync + 'static,
        <T as MakeTlsConnect<Socket>>::Stream: Send + Sync,
        <T as MakeTlsConnect<Socket>>::TlsConnect: Send,
        <<T as MakeTlsConnect<Socket>>::TlsConnect as TlsConnect<Socket>>::Future: Send,
        DE: FnOnce(&'a mut Body, &'a Parts, &'a Params<'b, 'c>) -> F1,
        F1: Future<Output = Result<InvalidArgumentResult<Option<I>>, ErrorAuditor>>,
        AP: FnOnce(&'a Pool<PostgresqlConnectionManager<T>>, &'a Pool<PostgresqlConnectionManager<T>>, &'a Pool<RedisConnectionManager>, Option<I>) -> F2,
        F2: Future<Output = Result<InvalidArgumentResult<UnifiedReport<O, P>>, ErrorAuditor>>,
        O: SerdeSerialize,
        P: SerdeSerialize,
        Serializer<SF>: Serialize,
    {
        if !Validator::<Parts>::is_valid(parts) {
            let response = Creator::<Response>::create_bad_request();

            Reactor::<InvalidArgument>::react(parts, &response, InvalidArgument::HttpHeader);

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
            Err(error) => {
                let response = Creator::<Response>::create_internal_server_error();

                Reactor::<ErrorAuditor>::react(parts, &response, error);

                return response;
            }
        };

        let action_processor_incoming_ = match incoming {
            InvalidArgumentResult::Ok {
                subject: action_processor_incoming__,
            } => action_processor_incoming__,
            InvalidArgumentResult::InvalidArgument {
                invalid_argument,
            } => {
                let response = Creator::<Response>::create_bad_request();

                Reactor::<InvalidArgument>::react(parts, &response, invalid_argument);

                return response;
            }
        };

        let unified_report = match action_processor(
            database_1_postgresql_connection_pool,
            database_2_postgresql_connection_pool,
            database_1_redis_connection_pool,
            action_processor_incoming_,
        )
        .await
        {
            Ok(unified_report_) => unified_report_,
            Err(error) => {
                let response = Creator::<Response>::create_internal_server_error();

                Reactor::<ErrorAuditor>::react(parts, &response, error);

                return response;
            }
        };

        let unified_report_ = match unified_report {
            InvalidArgumentResult::Ok {
                subject: unified_report__,
            } => unified_report__,
            InvalidArgumentResult::InvalidArgument {
                invalid_argument,
            } => {
                let response = Creator::<Response>::create_bad_request();

                Reactor::<InvalidArgument>::react(parts, &response, invalid_argument);

                return response;
            }
        };

        let data = match Serializer::<SF>::serialize(&unified_report_) {
            Ok(data_) => data_,
            Err(error) => {
                let response = Creator::<Response>::create_internal_server_error();

                Reactor::<ErrorAuditor>::react(parts, &response, error);

                return response;
            }
        };

        let response = Creator::<Response>::create_ok(data);

        Reactor::<Response>::react(parts, &response);

        return response;
    }
}
