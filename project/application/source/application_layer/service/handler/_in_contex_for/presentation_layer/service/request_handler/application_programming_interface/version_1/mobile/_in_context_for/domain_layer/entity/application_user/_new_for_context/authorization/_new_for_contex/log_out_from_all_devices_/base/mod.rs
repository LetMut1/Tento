use bb8_redis::RedisConnectionManager;
use bb8::Pool;
use bytes::Buf;
use crate::infrastructure_layer::error::base_error::base_error::BaseError;
use crate::presentation_layer::data_transfer_object::_in_context_for::presentation_layer::service::response_data_wrapper::_new_for_context::wrapped_response::WrappedResponseData;
use crate::presentation_layer::data_transfer_object::response_data::_in_context_for::presentation_layer::service::request_handler::application_programming_interface::version_1::mobile::_in_context_for::domain_layer::entity::application_user::_new_for_context::authorization::_new_for_context::log_out_from_all_devices_::base::Base as ResponseData;
use crate::presentation_layer::service::request_handler::application_programming_interface::version_1::mobile::_in_context_for::domain_layer::entity::application_user::_new_for_context::authorization::Authorization;
use http::request::Parts;
use http::StatusCode;
use hyper::Body;
use hyper::body::to_bytes;
use hyper::Request;

pub struct Base;

impl Base {
    pub async fn handle(
        redis_connection_pool: Pool<RedisConnectionManager>,
        parts: Parts
    ) -> Result<ResponseData, BaseError> {
        let request = Request::from_parts(parts, Body::empty());
        
        let response = Authorization::log_out_from_all_devices(request, redis_connection_pool).await;

        let response_data: ResponseData;

        let (
            response_parts,
            body
        ) = response.into_parts();

        if response_parts.status == StatusCode::OK {
            let bytes = to_bytes(body).await?;

            let wrapped_response = rmp_serde::from_read_ref::<'_, [u8], WrappedResponseData<()>>(bytes.chunk())?;

            response_data = (Some(wrapped_response), response_parts);
        } else {
            response_data = (None, response_parts);
        }

        return Ok(response_data);
    }
}