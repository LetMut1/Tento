use bb8_postgres::PostgresConnectionManager as PostgresqlConnectionManager;
use bb8_redis::RedisConnectionManager;
use bb8::Pool;
use bytes::Buf;
use crate::infrastructure_layer::error::base_error::base_error::BaseError;
use crate::presentation_layer::data_transfer_object::_in_context_for::presentation_layer::service::response_data_wrapper::_new_for_context::wrapped_response::WrappedResponseData;
use crate::presentation_layer::data_transfer_object::request_data::_in_context_for::presentation_layer::service::request_handler::application_programming_interface::version_1::mobile::_in_context_for::domain_layer::entity::application_user::_new_for_context::authorization::_new_for_context::reset_password_by_first_step::base::Base as RequestData;
use crate::presentation_layer::data_transfer_object::response_data::_in_context_for::presentation_layer::service::request_handler::application_programming_interface::version_1::mobile::_in_context_for::domain_layer::entity::application_user::_new_for_context::authorization::_new_for_context::reset_password_by_first_step::base::Base as ResponseData;
use crate::presentation_layer::service::request_handler::application_programming_interface::version_1::mobile::_in_context_for::domain_layer::entity::application_user::_new_for_context::authorization::Authorization;
use http::request::Parts as RequestParts;
use http::response::Parts as ResponseParts;
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
        request_parts: RequestParts,
        request_data: RequestData
    ) -> Result<(Option<WrappedResponseData<ResponseData>>, ResponseParts), BaseError> {
        let mut data: Vec<u8> = vec![];
        rmp_serde::encode::write(&mut data, &request_data)?;

        let request = Request::from_parts(request_parts, Body::from(data));
        
        let response = Authorization::reset_password_by_first_step(request, postgresql_connection_pool, redis_connection_pool).await;

        let result: (Option<WrappedResponseData<ResponseData>>, ResponseParts);

        let (
            response_parts,
            body
        ) = response.into_parts();

        if response_parts.status == StatusCode::OK {
            let bytes = to_bytes(body).await?;

            let wrapped_response = rmp_serde::from_read_ref::<'_, [u8], WrappedResponseData<ResponseData>>(bytes.chunk())?;

            result = (Some(wrapped_response), response_parts);
        } else {
            result = (None, response_parts);
        }

        return Ok(result);
    }
}