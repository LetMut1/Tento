use crate::application_layer::data::action_handler_result::ActionHandlerResult;
use crate::infrastructure_layer::data::error_auditor::BacktracePart;
use crate::infrastructure_layer::data::error_auditor::BaseError;
use crate::infrastructure_layer::data::error_auditor::ErrorAuditor;
use crate::infrastructure_layer::data::error_auditor::OtherError;
use crate::infrastructure_layer::data::error_auditor::RunTimeError;
use crate::infrastructure_layer::functionality::service::environment_configuration_resolver::EnvironmentConfigurationResolver;
use crate::presentation_layer::data::unified_report::UnifiedReport;
use extern_crate::bb8_postgres::PostgresConnectionManager as PostgresqlConnectionManager;
use extern_crate::bb8_redis::RedisConnectionManager;
use extern_crate::bb8::Pool;
use extern_crate::bytes::Buf;
use extern_crate::http::header;
use extern_crate::http::HeaderValue;
use extern_crate::http::request::Parts as HttpRequestParts;
use extern_crate::http::response::Parts as HttpResponseParts;
use extern_crate::http::StatusCode;
use extern_crate::hyper::Body;
use extern_crate::hyper::body::to_bytes;
use extern_crate::hyper::Request;
use extern_crate::hyper::Response;
use extern_crate::rmp_serde;
use extern_crate::serde::Deserialize;
use extern_crate::serde::Serialize;
use extern_crate::tokio_postgres::Socket;
use extern_crate::tokio_postgres::tls::MakeTlsConnect;
use extern_crate::tokio_postgres::tls::TlsConnect;
use std::clone::Clone;
use std::convert::From;
use std::future::Future;
use std::marker::Send;
use std::marker::Sync;

#[cfg(feature = "facilitate_non_automatic_functional_testing")]
pub struct ActionRoundParameterExtractor;

impl ActionRoundParameterExtractor {
    pub async fn handle<'a, T, FO, F, AHID, AHOD>(
        environment_configuration_resolver: &'a EnvironmentConfigurationResolver,
        core_postgresql_connection_pool: Pool<PostgresqlConnectionManager<T>>,
        authorization_postgresql_connection_pool: Pool<PostgresqlConnectionManager<T>>,
        redis_connection_pool: Pool<RedisConnectionManager>,
        incoming: Incoming<AHID>,
        action: FO
    ) -> Result<ActionHandlerResult<Outcoming<AHOD>>, ErrorAuditor>
    where
        T: MakeTlsConnect<Socket> + Clone + Send + Sync + 'static,
        <T as MakeTlsConnect<Socket>>::Stream: Send + Sync,
        <T as MakeTlsConnect<Socket>>::TlsConnect: Send,
        <<T as MakeTlsConnect<Socket>>::TlsConnect as TlsConnect<Socket>>::Future: Send,
        FO: FnOnce(
            &'a EnvironmentConfigurationResolver,
            Request<Body>,
            Pool<PostgresqlConnectionManager<T>>,
            Pool<PostgresqlConnectionManager<T>>,
            Pool<RedisConnectionManager>
        ) -> F,
        F: Future<Output = Response<Body>>,
        AHID: Serialize + for<'de> Deserialize<'de>,
        AHOD: Serialize + for<'de> Deserialize<'de>
    {
        let (
            mut request_parts,
            convertible_data
        ) = incoming.into_inner();

        let mut data: Vec<u8> = vec![];
        if let Err(error) = rmp_serde::encode::write(&mut data, &convertible_data) {
            return Err(
                ErrorAuditor::new(
                    BaseError::RunTimeError { run_time_error: RunTimeError::OtherError { other_error: OtherError::new(error) } },
                    BacktracePart::new(line!(), file!(), None)
                )
            );
        }

        let mut header_map = request_parts.headers;
        header_map.remove(header::CONTENT_LENGTH);
        header_map.append(header::CONTENT_LENGTH, HeaderValue::from(data.len() as u64));
        request_parts.headers = header_map;

        let request = Request::from_parts(request_parts, Body::from(data));

        let response = action(
            environment_configuration_resolver, request, core_postgresql_connection_pool, authorization_postgresql_connection_pool, redis_connection_pool
        ).await;

        let outcoming: Outcoming<AHOD>;

        let (
            response_parts,
            body
        ) = response.into_parts();

        if response_parts.status == StatusCode::OK {
            match to_bytes(body).await {
                Ok(bytes) => {
                    match rmp_serde::from_read_ref::<'_, [u8], UnifiedReport<AHOD>>(bytes.chunk()) {
                        Ok(unified_report) => {
                            outcoming = Outcoming::new(response_parts, Some(unified_report));
                        }
                        Err(error) => {
                            return Err(
                                ErrorAuditor::new(
                                    BaseError::RunTimeError { run_time_error: RunTimeError::OtherError { other_error: OtherError::new(error) } },
                                    BacktracePart::new(line!(), file!(), None)
                                )
                            );
                        }
                    }
                }
                Err(error) => {
                    return Err(
                        ErrorAuditor::new(
                            BaseError::RunTimeError { run_time_error: RunTimeError::OtherError { other_error: OtherError::new(error) } },
                            BacktracePart::new(line!(), file!(), None)
                        )
                    );
                }
            }
        } else {
            outcoming = Outcoming::new(response_parts, None);
        }

        return Ok(ActionHandlerResult::new_with_outcoming(outcoming));
    }
}

#[cfg(feature = "facilitate_non_automatic_functional_testing")]
pub struct Incoming<T> {
    parts: HttpRequestParts,
    convertible_data: T
}

impl<T> Incoming<T> {
    pub fn new(
        parts: HttpRequestParts,
        convertible_data: T
    ) -> Self {
        return Self {
            parts,
            convertible_data
        };
    }

    pub fn into_inner(
        self
    ) -> (HttpRequestParts, T) {
        return (
            self.parts,
            self.convertible_data
        );
    }
}

#[cfg(feature = "facilitate_non_automatic_functional_testing")]
pub struct Outcoming<T> {
    parts: HttpResponseParts,
    unified_report: Option<UnifiedReport<T>>
}

impl<T> Outcoming<T> {
    pub fn new(
        parts: HttpResponseParts,
        unified_report: Option<UnifiedReport<T>>
    ) -> Self {
        return Self {
            parts,
            unified_report
        };
    }

    pub fn into_inner(
        self
    ) -> (HttpResponseParts, Option<UnifiedReport<T>>) {
        return (
            self.parts,
            self.unified_report,
        );
    }
}