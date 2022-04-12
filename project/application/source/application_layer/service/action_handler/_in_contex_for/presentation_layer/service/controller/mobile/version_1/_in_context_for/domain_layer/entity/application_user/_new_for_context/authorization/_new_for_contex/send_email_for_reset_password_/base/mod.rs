use bb8_postgres::PostgresConnectionManager as PostgresqlConnectionManager;
use bb8_redis::RedisConnectionManager;
use bb8::Pool;
use bytes::Buf;
use crate::infrastructure_layer::error::error_aggregator::error_aggregator::ErrorAggregator;
use crate::presentation_layer::data_transfer_object::_in_context_for::presentation_layer::service::controller::_new_for_context::endpoint_response::endpoint_response::EndpointResponse;
use crate::presentation_layer::data_transfer_object::request_data::_in_context_for::presentation_layer::service::controller::mobile::version_1::_in_context_for::domain_layer::entity::application_user::_new_for_context::authorization::_new_for_context::send_email_for_reset_password_::base::Base as RequestData;
use crate::presentation_layer::data_transfer_object::response_data::_in_context_for::presentation_layer::service::controller::mobile::version_1::_in_context_for::domain_layer::entity::application_user::_new_for_context::authorization::_new_for_context::send_email_for_reset_password_::base::Base as ResponseData;
use crate::presentation_layer::service::controller::mobile::version_1::_in_context_for::domain_layer::entity::application_user::_new_for_context::authorization::Authorization;
use http::header;
use http::HeaderValue;
use http::StatusCode;
use hyper::Body;
use hyper::body::to_bytes;
use hyper::Request;
use std::convert::From;
use tokio_postgres::NoTls;

pub struct Base;

impl Base {
    pub async fn handle(
        postgresql_connection_pool: Pool<PostgresqlConnectionManager<NoTls>>,
        redis_connection_pool: Pool<RedisConnectionManager>,
        request_data: RequestData
    ) -> Result<ResponseData, ErrorAggregator> {
        let (
            mut request_parts,
            convertible_data
        ) = request_data.into_inner();

        let mut data: Vec<u8> = vec![];
        rmp_serde::encode::write(&mut data, &convertible_data)?;

        let mut header_map = request_parts.headers;
        header_map.remove(header::CONTENT_LENGTH);
        header_map.append(header::CONTENT_LENGTH, HeaderValue::from(data.len() as u64));
        request_parts.headers = header_map;

        let request = Request::from_parts(request_parts, Body::from(data));
        
        let response = Authorization::send_email_for_reset_password(request, postgresql_connection_pool, redis_connection_pool).await;

        let response_data: ResponseData;

        let (
            response_parts,
            body
        ) = response.into_parts();

        if response_parts.status == StatusCode::OK {
            let bytes = to_bytes(body).await?;

            let endpoint_response = rmp_serde::from_read_ref::<'_, [u8], EndpointResponse<()>>(bytes.chunk())?;

            response_data = ResponseData::new(response_parts, Some(endpoint_response));
        } else {
            response_data = ResponseData::new(response_parts, None);
        }

        return Ok(response_data);
    }
}