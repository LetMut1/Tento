use bb8_postgres::PostgresConnectionManager as PostgresqlConnectionManager;
use bb8_redis::RedisConnectionManager;
use bb8::Pool;
use bytes::Buf;
use crate::infrastructure_layer::error::error_auditor::_component::error_aggregator::_component::run_time_error::_component::other_error::OtherError;
use crate::infrastructure_layer::error::error_auditor::_component::error_aggregator::_component::run_time_error::run_time_error::RunTimeError;
use crate::infrastructure_layer::error::error_auditor::_component::error_aggregator::error_aggregator::ErrorAggregator;
use crate::infrastructure_layer::error::error_auditor::_component::simple_backtrace::_component::backtrace_part::BacktracePart;
use crate::infrastructure_layer::error::error_auditor::error_auditor::ErrorAuditor;
use crate::infrastructure_layer::service::environment_configuration_resolver::EnvironmentConfigurationResolver;
use crate::presentation_layer::data_transfer_object::_in_context_for::presentation_layer::service::controller::_new_for_context::endpoint_response::endpoint_response::EndpointResponse;
use crate::presentation_layer::data_transfer_object::request_data::_in_context_for::presentation_layer::service::controller::mobile::version_1::_in_context_for::domain_layer::entity::application_user::_new_for_context::authorization::_new_for_context::send_email_for_log_in_::base::Base as RequestData;
use crate::presentation_layer::data_transfer_object::response_data::_in_context_for::presentation_layer::service::controller::mobile::version_1::_in_context_for::domain_layer::entity::application_user::_new_for_context::authorization::_new_for_context::send_email_for_log_in_::base::Base as ResponseData;
use crate::presentation_layer::service::controller::mobile::version_1::_in_context_for::domain_layer::entity::application_user::_new_for_context::authorization::Authorization;
use http::header;
use http::HeaderValue;
use http::StatusCode;
use hyper::Body;
use hyper::body::to_bytes;
use hyper::Request;
use std::clone::Clone;
use std::convert::From;
use std::marker::Send;
use std::marker::Sync;
use tokio_postgres::Socket;
use tokio_postgres::tls::MakeTlsConnect;
use tokio_postgres::tls::TlsConnect;

pub struct Base;

impl Base {
    pub async fn handle<'a, T>(
        environment_configuration_resolver: &'a EnvironmentConfigurationResolver,
        postgresql_connection_pool: Pool<PostgresqlConnectionManager<T>>,
        redis_connection_pool: Pool<RedisConnectionManager>,
        request_data: RequestData
    ) -> Result<ResponseData, ErrorAuditor>
    where 
        T: MakeTlsConnect<Socket> + Clone + Send + Sync + 'static,
        <T as MakeTlsConnect<Socket>>::Stream: Send + Sync,
        <T as MakeTlsConnect<Socket>>::TlsConnect: Send,
        <<T as MakeTlsConnect<Socket>>::TlsConnect as TlsConnect<Socket>>::Future: Send
    {
        let (
            mut request_parts,
            convertible_data
        ) = request_data.into_inner();

        let mut data: Vec<u8> = vec![];
        if let Err(error) = rmp_serde::encode::write(&mut data, &convertible_data) {
            return Err(
                ErrorAuditor::new(
                    ErrorAggregator::RunTimeError { run_time_error: RunTimeError::OtherError { other_error: OtherError::new(error) } },
                    BacktracePart::new(line!(), file!(), None)
                )
            );
        }

        let mut header_map = request_parts.headers;
        header_map.remove(header::CONTENT_LENGTH);
        header_map.append(header::CONTENT_LENGTH, HeaderValue::from(data.len() as u64));
        request_parts.headers = header_map;

        let request = Request::from_parts(request_parts, Body::from(data));
        
        let response = Authorization::send_email_for_log_in(environment_configuration_resolver, request, postgresql_connection_pool, redis_connection_pool).await;

        let response_data: ResponseData;

        let (
            response_parts,
            body
        ) = response.into_parts();

        if response_parts.status == StatusCode::OK {
            match to_bytes(body).await {
                Ok(bytes) => {
                    match rmp_serde::from_read_ref::<'_, [u8], EndpointResponse<()>>(bytes.chunk()) {
                        Ok(endpoint_response) => {
                            response_data = ResponseData::new(response_parts, Some(endpoint_response));
                        }
                        Err(error) => {
                            return Err(
                                ErrorAuditor::new(
                                    ErrorAggregator::RunTimeError { run_time_error: RunTimeError::OtherError { other_error: OtherError::new(error) } },
                                    BacktracePart::new(line!(), file!(), None)
                                )
                            );
                        }
                    }
                }
                Err(error) => {
                    return Err(
                        ErrorAuditor::new(
                            ErrorAggregator::RunTimeError { run_time_error: RunTimeError::OtherError { other_error: OtherError::new(error) } },
                            BacktracePart::new(line!(), file!(), None)
                        )
                    );
                }
            }
        } else {
            response_data = ResponseData::new(response_parts, None);
        }

        return Ok(response_data);
    }
}