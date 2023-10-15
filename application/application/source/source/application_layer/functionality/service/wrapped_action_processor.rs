use crate::application_layer::data::unified_report::UnifiedReport;
use crate::infrastructure_layer::data::control_type::Response;
use crate::infrastructure_layer::data::error_auditor::BacktracePart;
use crate::infrastructure_layer::data::error_auditor::Error;
use crate::infrastructure_layer::data::error_auditor::ErrorAuditor_;
use crate::infrastructure_layer::data::error_auditor::Other;
use crate::infrastructure_layer::data::error_auditor::Runtime;
use crate::infrastructure_layer::functionality::service::creator::Creator;
use crate::infrastructure_layer::functionality::service::serializer::Serialize;
use crate::infrastructure_layer::functionality::service::serializer::Serializer;
use crate::infrastructure_layer::functionality::service::validator::Validator;
use bb8::Pool;
use bb8_postgres::PostgresConnectionManager as PostgresqlConnectionManager;
use bb8_redis::RedisConnectionManager;
use bytes::Buf;
use http::header;
use http::response::Parts as ResponseParts;
use http::HeaderValue;
use http::StatusCode;
use hyper::body::to_bytes;
use hyper::Body;
use serde::Deserialize;
use serde::Serialize as SerdeSerialize;
use std::clone::Clone;
use std::convert::From;
use std::future::Future;
use std::marker::Send;
use std::marker::Sync;
use std::ops::FnOnce;
use tokio_postgres::tls::MakeTlsConnect;
use tokio_postgres::tls::TlsConnect;
use tokio_postgres::Socket;
use matchit::Params;
use http::request::Parts as RequestParts;

#[cfg(feature = "manual_testing")]
pub struct WrappedActionProcessor;

#[cfg(feature = "manual_testing")]
impl WrappedActionProcessor {
    pub async fn process<'a, 'b, 'c, T, A, F, API, APO, APP, SF, WSF>(
        body: &'a mut Body,
        // TODO https://github.com/rust-lang/rust/issues/102211#issuecomment-1513931928
        body_: &'a mut Body,
        request_parts: &'a mut RequestParts,
        route_parameters: &'a Params<'b, 'c>,
        database_1_postgresql_connection_pool: &'a Pool<PostgresqlConnectionManager<T>>,
        database_2_postgresql_connection_pool: &'a Pool<PostgresqlConnectionManager<T>>,
        database_1_redis_connection_pool: &'a Pool<RedisConnectionManager>,
        action: A,
    ) -> Response
    where
        T: MakeTlsConnect<Socket> + Clone + Send + Sync + 'static,
        <T as MakeTlsConnect<Socket>>::Stream: Send + Sync,
        <T as MakeTlsConnect<Socket>>::TlsConnect: Send,
        <<T as MakeTlsConnect<Socket>>::TlsConnect as TlsConnect<Socket>>::Future: Send,
        A: FnOnce(&'a mut Body, &'a RequestParts, &'a Params<'b, 'c>, &'a Pool<PostgresqlConnectionManager<T>>, &'a Pool<PostgresqlConnectionManager<T>>, &'a Pool<RedisConnectionManager>) -> F,
        F: Future<Output = Response>,
        API: SerdeSerialize + for<'de> Deserialize<'de>,
        APO: SerdeSerialize + for<'de> Deserialize<'de>,
        APP: SerdeSerialize + for<'de> Deserialize<'de>,
        Serializer<SF>: Serialize,
        Serializer<WSF>: Serialize,
    {
        if !Validator::<RequestParts>::is_valid(request_parts) {
            return Creator::<Response>::create_bad_request();
        }

        let bytes = match to_bytes(body).await {
            Ok(bytes_) => bytes_,
            Err(_) => {
                return Creator::<Response>::create_internal_server_error();
            }
        };

        let action_processor_incoming = match Serializer::<SF>::deserialize::<API>(bytes.chunk()) {
            Ok(action_processor_incoming_) => action_processor_incoming_,
            Err(_) => {
                return Creator::<Response>::create_internal_server_error();
            }
        };

        let result_ = match ActionDelegator::delegate::<'_, '_, '_, _, _, _, _, APO, APP, WSF>(
            body_,
            request_parts,
            route_parameters,
            database_1_postgresql_connection_pool,
            database_2_postgresql_connection_pool,
            database_1_redis_connection_pool,
            action_processor_incoming,
            action,
        )
        .await
        {
            Ok(result__) => result__,
            Err(_) => {
                return Creator::<Response>::create_internal_server_error();
            }
        };

        let unified_report = match result_.unified_report {
            Some(unified_report_) => unified_report_,
            None => {
                return Creator::<Response>::create_from_response_parts(
                    result_.response_parts,
                    None,
                );
            }
        };

        let data = match Serializer::<SF>::serialize(&unified_report) {
            Ok(data_) => data_,
            Err(_) => {
                return Creator::<Response>::create_internal_server_error();
            }
        };

        return Creator::<Response>::create_from_response_parts(
            result_.response_parts,
            Some(data),
        );
    }
}

#[cfg(feature = "manual_testing")]
struct ActionDelegator;

#[cfg(feature = "manual_testing")]
impl ActionDelegator {
    async fn delegate<'a, 'b, 'c, T, A, F, API, APO, APP, SF,>(
        // TODO https://github.com/rust-lang/rust/issues/102211#issuecomment-1513931928
        request_body: &'a mut Body,
        request_parts: &'a mut RequestParts,
        route_parameters: &'a Params<'b, 'c>,
        database_1_postgresql_connection_pool: &'a Pool<PostgresqlConnectionManager<T>>,
        database_2_postgresql_connection_pool: &'a Pool<PostgresqlConnectionManager<T>>,
        database_1_redis_connection_pool: &'a Pool<RedisConnectionManager>,
        action_processor_incoming: API,
        action: A,
    ) -> Result<Result_<APO, APP>, ErrorAuditor_>
    where
        T: MakeTlsConnect<Socket> + Clone + Send + Sync + 'static,
        <T as MakeTlsConnect<Socket>>::Stream: Send + Sync,
        <T as MakeTlsConnect<Socket>>::TlsConnect: Send,
        <<T as MakeTlsConnect<Socket>>::TlsConnect as TlsConnect<Socket>>::Future: Send,
        A: FnOnce(&'a mut Body, &'a RequestParts, &'a Params<'b, 'c>, &'a Pool<PostgresqlConnectionManager<T>>, &'a Pool<PostgresqlConnectionManager<T>>, &'a Pool<RedisConnectionManager>) -> F,
        F: Future<Output = Response>,
        API: SerdeSerialize + for<'de> Deserialize<'de>,
        APO: SerdeSerialize + for<'de> Deserialize<'de>,
        APP: SerdeSerialize + for<'de> Deserialize<'de>,
        Serializer<SF>: Serialize,
    {
        let data = match Serializer::<SF>::serialize(&action_processor_incoming) {
            Ok(data_) => data_,
            Err(mut error) => {
                error.add_backtrace_part(
                    BacktracePart::new(
                        line!(),
                        file!(),
                        None,
                    ),
                );

                return Err(error);
            }
        };

        request_parts.headers.remove(header::CONTENT_LENGTH);

        request_parts.headers.append(
            header::CONTENT_LENGTH,
            HeaderValue::from(data.len() as u64),
        );

        let request_body_ = Body::from(data);

        // TODO https://github.com/rust-lang/rust/issues/102211#issuecomment-1513931928
        // Right way.
        // let response = {
            // request_body: &'a mut Body - нужно убрать из сигнатуры
            // При наличии HRTB -  A: for<'d, 'e, 'f> FnOnce(&'d mut Body, &'d RequestParts, &'d Params<'e, 'f>, &'d Pool<PostgresqlConnectionManager<T>>, &'d Pool<PostgresqlConnectionManager<T>>, &'d Pool<RedisConnectionManager>) -> F,

            // let response = action(
            //     &mut request_body_,
            //     request_parts,
            //     route_parameters,
            //     database_1_postgresql_connection_pool,
            //     database_2_postgresql_connection_pool,
            //     database_1_redis_connection_pool,
            // )
            // .await;
        // };

        // TODO https://github.com/rust-lang/rust/issues/102211#issuecomment-1513931928
        //  Bad way. Very bad but worked way. It is neede to write Right way after bug fixing.
        let response = {
            *request_body = request_body_;

            action(
                request_body,
                request_parts,
                route_parameters,
                database_1_postgresql_connection_pool,
                database_2_postgresql_connection_pool,
                database_1_redis_connection_pool,
            )
            .await
        };

        let (response_parts, response_body) = response.into_parts();

        let result_ = if response_parts.status == StatusCode::OK {
            let bytes = match to_bytes(response_body).await {
                Ok(bytes_) => bytes_,
                Err(error) => {
                    return Err(
                        ErrorAuditor_::new(
                            Error::Runtime {
                                runtime: Runtime::Other {
                                    other: Other::new(error),
                                },
                            },
                            BacktracePart::new(
                                line!(),
                                file!(),
                                None,
                            ),
                        ),
                    );
                }
            };

            let unified_report = match Serializer::<SF>::deserialize::<'_, UnifiedReport<APO, APP>>(bytes.chunk()) {
                Ok(unified_report_) => unified_report_,
                Err(mut error) => {
                    error.add_backtrace_part(
                        BacktracePart::new(
                            line!(),
                            file!(),
                            None,
                        ),
                    );

                    return Err(error);
                }
            };

            Result_ {
                response_parts,
                unified_report: Some(unified_report),
            }
        } else {
            Result_ {
                response_parts,
                unified_report: None,
            }
        };

        return Ok(result_);
    }
}

#[cfg(feature = "manual_testing")]
struct Result_<T, P> {
    response_parts: ResponseParts,
    unified_report: Option<UnifiedReport<T, P>>,
}
