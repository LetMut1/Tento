use crate::application_layer::data::unified_report::UnifiedReport;
use crate::infrastructure_layer::data::control_type::Request;
use crate::infrastructure_layer::data::control_type::Response;
use crate::infrastructure_layer::data::error_auditor::BacktracePart;
use crate::infrastructure_layer::data::error_auditor::Error;
use crate::infrastructure_layer::data::error_auditor::ErrorAuditor;
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
use http::response::Parts;
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

#[cfg(feature = "manual_testing")]
pub struct WrappedActionProcessor;

#[cfg(feature = "manual_testing")]
impl WrappedActionProcessor {
    pub async fn process<'a, SF, WSF, T, WA, F, API, APO, APP>(
        mut request: Request,
        database_1_postgresql_connection_pool: &'a Pool<PostgresqlConnectionManager<T>>,
        database_2_postgresql_connection_pool: &'a Pool<PostgresqlConnectionManager<T>>,
        database_1_redis_connection_pool: &'a Pool<RedisConnectionManager>,
        action: WA,
    ) -> Response
    where
        Serializer<SF>: Serialize,
        Serializer<WSF>: Serialize,
        T: MakeTlsConnect<Socket> + Clone + Send + Sync + 'static,
        <T as MakeTlsConnect<Socket>>::Stream: Send + Sync,
        <T as MakeTlsConnect<Socket>>::TlsConnect: Send,
        <<T as MakeTlsConnect<Socket>>::TlsConnect as TlsConnect<Socket>>::Future: Send,
        WA: FnOnce(Request, &'a Pool<PostgresqlConnectionManager<T>>, &'a Pool<PostgresqlConnectionManager<T>>, &'a Pool<RedisConnectionManager>) -> F,
        F: Future<Output = Response>,
        API: SerdeSerialize + for<'de> Deserialize<'de>,
        APO: SerdeSerialize + for<'de> Deserialize<'de>,
        APP: SerdeSerialize + for<'de> Deserialize<'de>,
    {
        if !Validator::<Request>::is_valid(&request) {
            return Creator::<Response>::create_bad_request();
        }

        let bytes = match to_bytes(request.body_mut()).await {
            Ok(bytes_) => bytes_,
            Err(_) => {
                return Creator::<Response>::create_internal_server_error();
            }
        };

        let incoming = match Serializer::<SF>::deserialize::<API>(bytes.chunk()) {
            Ok(wrapped_incoming_) => wrapped_incoming_,
            Err(_) => {
                return Creator::<Response>::create_internal_server_error();
            }
        };

        let action_processing_delegator_result = match ActionDelegator::delegate::<'_, WSF, _, _, _, API, APO, APP>(
            database_1_postgresql_connection_pool,
            database_2_postgresql_connection_pool,
            database_1_redis_connection_pool,
            ConvertibleParts {
                request,
                action_processor_incoming: incoming,
            },
            action,
        )
        .await
        {
            Ok(action_processor_result_) => action_processor_result_,
            Err(_) => {
                return Creator::<Response>::create_internal_server_error();
            }
        };

        let unified_report = match action_processing_delegator_result.unified_report {
            Some(unified_report_) => unified_report_,
            None => {
                return Creator::<Response>::create_from_response_parts(
                    action_processing_delegator_result.response_parts,
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
            action_processing_delegator_result.response_parts,
            Some(data),
        );
    }
}

#[cfg(feature = "manual_testing")]
struct ActionDelegator;

#[cfg(feature = "manual_testing")]
impl ActionDelegator {
    async fn delegate<'a, SF, T, A, F, API, APO, APP>(
        database_1_postgresql_connection_pool: &'a Pool<PostgresqlConnectionManager<T>>,
        database_2_postgresql_connection_pool: &'a Pool<PostgresqlConnectionManager<T>>,
        database_1_redis_connection_pool: &'a Pool<RedisConnectionManager>,
        convertible_parts: ConvertibleParts<API>,
        action: A,
    ) -> Result<ActionProcessingDelegatorResult<APO, APP>, ErrorAuditor>
    where
        Serializer<SF>: Serialize,
        T: MakeTlsConnect<Socket> + Clone + Send + Sync + 'static,
        <T as MakeTlsConnect<Socket>>::Stream: Send + Sync,
        <T as MakeTlsConnect<Socket>>::TlsConnect: Send,
        <<T as MakeTlsConnect<Socket>>::TlsConnect as TlsConnect<Socket>>::Future: Send,
        A: FnOnce(Request, &'a Pool<PostgresqlConnectionManager<T>>, &'a Pool<PostgresqlConnectionManager<T>>, &'a Pool<RedisConnectionManager>) -> F,
        F: Future<Output = Response>,
        API: SerdeSerialize + for<'de> Deserialize<'de>,
        APO: SerdeSerialize + for<'de> Deserialize<'de>,
        APP: SerdeSerialize + for<'de> Deserialize<'de>,
    {
        let data = match Serializer::<SF>::serialize(&convertible_parts.action_processor_incoming) {
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

        let mut request_parts = convertible_parts.request.into_parts().0;

        request_parts.headers.remove(header::CONTENT_LENGTH);

        request_parts.headers.append(
            header::CONTENT_LENGTH,
            HeaderValue::from(data.len() as u64),
        );

        let request = Request::from_parts(
            request_parts,
            Body::from(data),
        );

        let response = action(
            request,
            database_1_postgresql_connection_pool,
            database_2_postgresql_connection_pool,
            database_1_redis_connection_pool,
        )
        .await;

        let (response_parts, body) = response.into_parts();

        let action_processing_delegator_result = if response_parts.status == StatusCode::OK {
            let bytes = match to_bytes(body).await {
                Ok(bytes_) => bytes_,
                Err(error) => {
                    return Err(
                        ErrorAuditor::new(
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

            ActionProcessingDelegatorResult {
                response_parts,
                unified_report: Some(unified_report),
            }
        } else {
            ActionProcessingDelegatorResult {
                response_parts,
                unified_report: None,
            }
        };

        return Ok(action_processing_delegator_result);
    }
}

#[cfg(feature = "manual_testing")]
struct ConvertibleParts<T>
where
    T: SerdeSerialize + for<'de> Deserialize<'de>,
{
    request: Request,
    action_processor_incoming: T,
}

#[cfg(feature = "manual_testing")]
struct ActionProcessingDelegatorResult<T, P>
where
    T: SerdeSerialize + for<'de> Deserialize<'de>,
    P: SerdeSerialize + for<'de> Deserialize<'de>,
{
    response_parts: Parts,
    unified_report: Option<UnifiedReport<T, P>>,
}
