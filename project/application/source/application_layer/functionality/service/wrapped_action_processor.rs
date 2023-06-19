use crate::infrastructure_layer::data::control_type_registry::Request;
use crate::infrastructure_layer::data::control_type_registry::Response;
use crate::infrastructure_layer::data::environment_configuration::EnvironmentConfiguration;
use crate::infrastructure_layer::data::error_auditor::BacktracePart;
use crate::infrastructure_layer::data::error_auditor::BaseError;
use crate::infrastructure_layer::data::error_auditor::ErrorAuditor;
use crate::infrastructure_layer::data::error_auditor::OtherError;
use crate::infrastructure_layer::data::error_auditor::RuntimeError;
use crate::infrastructure_layer::functionality::service::creator::Creator;
use crate::infrastructure_layer::functionality::service::serializer::Serialize;
use crate::infrastructure_layer::functionality::service::serializer::Serializer;
use crate::infrastructure_layer::functionality::service::validator::Validator;
use crate::presentation_layer::data::unified_report::UnifiedReport;
use extern_crate::bb8_postgres::PostgresConnectionManager as PostgresqlConnectionManager;
use extern_crate::bb8_redis::RedisConnectionManager;
use extern_crate::bb8::Pool;
use extern_crate::bytes::Buf;
use extern_crate::http::header;
use extern_crate::http::HeaderValue;
use extern_crate::http::response::Parts;
use extern_crate::http::StatusCode;
use extern_crate::hyper::Body;
use extern_crate::hyper::body::to_bytes;
use extern_crate::serde::Deserialize;
use extern_crate::serde::Serialize as SerdeSerialize;
use extern_crate::tokio_postgres::Socket;
use extern_crate::tokio_postgres::tls::MakeTlsConnect;
use extern_crate::tokio_postgres::tls::TlsConnect;
use std::clone::Clone;
use std::convert::From;
use std::future::Future;
use std::marker::Send;
use std::marker::Sync;
use std::ops::FnOnce;

#[cfg(feature = "facilitate_non_automatic_functional_testing")]
pub struct WrappedActionProcessor;

#[cfg(feature = "facilitate_non_automatic_functional_testing")]
impl WrappedActionProcessor {
    pub async fn process<'a, SF, WSF, T, WA, F, API, APO, APP>(
        environment_configuration: &'a EnvironmentConfiguration,
        mut request: Request,
        database_1_postgresql_connection_pool: &'a Pool<PostgresqlConnectionManager<T>>,
        database_2_postgresql_connection_pool: &'a Pool<PostgresqlConnectionManager<T>>,
        redis_connection_pool: &'a Pool<RedisConnectionManager>,
        action: WA
    ) -> Response
    where
        Serializer<SF>: Serialize,
        Serializer<WSF>: Serialize,
        T: MakeTlsConnect<Socket> + Clone + Send + Sync + 'static,
        <T as MakeTlsConnect<Socket>>::Stream: Send + Sync,
        <T as MakeTlsConnect<Socket>>::TlsConnect: Send,
        <<T as MakeTlsConnect<Socket>>::TlsConnect as TlsConnect<Socket>>::Future: Send,
        WA: FnOnce(
            &'a EnvironmentConfiguration,
            Request,
            &'a Pool<PostgresqlConnectionManager<T>>,
            &'a Pool<PostgresqlConnectionManager<T>>,
            &'a Pool<RedisConnectionManager>
        ) -> F,
        F: Future<Output = Response>,
        API: SerdeSerialize + for<'de> Deserialize<'de>,
        APO: SerdeSerialize + for<'de> Deserialize<'de>,
        APP: SerdeSerialize + for<'de> Deserialize<'de>
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
            environment_configuration,
            database_1_postgresql_connection_pool,
            database_2_postgresql_connection_pool,
            redis_connection_pool,
            ConvertibleParts {
                request,
                action_processor_incoming: incoming
            },
            action
        ).await {
            Ok(action_processor_result_) => action_processor_result_,
            Err(_) => {
                return Creator::<Response>::create_internal_server_error();
            }
        };

        let unified_report = match action_processing_delegator_result.unified_report {
            Some(unified_report_) => unified_report_,
            None => {
                return Creator::<Response>::create_from_response_parts(action_processing_delegator_result.response_parts, None);
            }
        };

        let data = match Serializer::<SF>::serialize(&unified_report) {
            Ok(data_) => data_,
            Err(_) => {
                return Creator::<Response>::create_internal_server_error();
            }
        };

        return Creator::<Response>::create_from_response_parts(action_processing_delegator_result.response_parts, Some(data));
    }
}


#[cfg(feature = "facilitate_non_automatic_functional_testing")]
struct ActionDelegator;

#[cfg(feature = "facilitate_non_automatic_functional_testing")]
impl ActionDelegator {
    async fn delegate<'a, SF, T, A, F, API, APO, APP>(
        environment_configuration: &'a EnvironmentConfiguration,
        database_1_postgresql_connection_pool: &'a Pool<PostgresqlConnectionManager<T>>,
        database_2_postgresql_connection_pool: &'a Pool<PostgresqlConnectionManager<T>>,
        redis_connection_pool: &'a Pool<RedisConnectionManager>,
        incoming: ConvertibleParts<API>,
        action: A
    ) -> Result<ActionProcessingDelegatorResult<APO, APP>, ErrorAuditor>
    where
        Serializer<SF>: Serialize,
        T: MakeTlsConnect<Socket> + Clone + Send + Sync + 'static,
        <T as MakeTlsConnect<Socket>>::Stream: Send + Sync,
        <T as MakeTlsConnect<Socket>>::TlsConnect: Send,
        <<T as MakeTlsConnect<Socket>>::TlsConnect as TlsConnect<Socket>>::Future: Send,
        A: FnOnce(
            &'a EnvironmentConfiguration,
            Request,
            &'a Pool<PostgresqlConnectionManager<T>>,
            &'a Pool<PostgresqlConnectionManager<T>>,
            &'a Pool<RedisConnectionManager>
        ) -> F,
        F: Future<Output = Response>,
        API: SerdeSerialize + for<'de> Deserialize<'de>,
        APO: SerdeSerialize + for<'de> Deserialize<'de>,
        APP: SerdeSerialize + for<'de> Deserialize<'de>
    {
        let data = match Serializer::<SF>::serialize(&incoming.action_processor_incoming) {
            Ok(data_) => data_,
            Err(mut error) => {
                error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                return Err(error);
            }
        };

        let mut request_parts = incoming.request.into_parts().0;

        request_parts.headers.remove(header::CONTENT_LENGTH);

        request_parts.headers.append(header::CONTENT_LENGTH, HeaderValue::from(data.len() as u64));

        let request = Request::from_parts(request_parts, Body::from(data));

        let response = action(
            environment_configuration,
            request,
            database_1_postgresql_connection_pool,
            database_2_postgresql_connection_pool,
            redis_connection_pool
        ).await;

        let (
            response_parts,
            body
        ) = response.into_parts();

        let action_processing_delegator_result = if response_parts.status == StatusCode::OK {
            let bytes = match to_bytes(body).await {
                Ok(bytes_) => bytes_,
                Err(error) => {
                    return Err(
                        ErrorAuditor::new(
                            BaseError::RuntimeError { runtime_error: RuntimeError::OtherError { other_error: OtherError::new(error) } },
                            BacktracePart::new(line!(), file!(), None)
                        )
                    );
                }
            };

            let unified_report = match Serializer::<SF>::deserialize::<'_, UnifiedReport<APO, APP>>(bytes.chunk()) {
                Ok(unified_report_) => unified_report_,
                Err(mut error) => {
                    error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                    return Err(error);
                }
            };

            ActionProcessingDelegatorResult { response_parts, unified_report: Some(unified_report) }
        } else {
            ActionProcessingDelegatorResult { response_parts, unified_report: None }
        };

        return Ok(action_processing_delegator_result);
    }
}

#[cfg(feature = "facilitate_non_automatic_functional_testing")]
struct ConvertibleParts<T>
where
    T: SerdeSerialize + for<'de> Deserialize<'de>
{
    request: Request,
    action_processor_incoming: T
}

#[cfg(feature = "facilitate_non_automatic_functional_testing")]
struct ActionProcessingDelegatorResult<T, P>
where
    T: SerdeSerialize + for<'de> Deserialize<'de>,
    P: SerdeSerialize + for<'de> Deserialize<'de>
{
    response_parts: Parts,
    unified_report: Option<UnifiedReport<T, P>>
}