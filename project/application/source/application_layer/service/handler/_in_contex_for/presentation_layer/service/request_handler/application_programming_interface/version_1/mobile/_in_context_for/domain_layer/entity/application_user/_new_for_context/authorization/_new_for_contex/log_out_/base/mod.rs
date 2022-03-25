use bb8_redis::RedisConnectionManager;
use bb8::Pool;
use bytes::Buf;
use crate::infrastructure_layer::error::base_error::base_error::BaseError;
use crate::presentation_layer::data_transfer_object::_in_context_for::presentation_layer::service::response_data_wrapper::_new_for_context::wrapped_response::WrappedResponseData;
use crate::presentation_layer::data_transfer_object::request_data::_in_context_for::presentation_layer::service::request_handler::application_programming_interface::version_1::mobile::_in_context_for::domain_layer::entity::application_user::_new_for_context::authorization::_new_for_context::log_out_::base::Base as RequestData;
use crate::presentation_layer::data_transfer_object::response_data::_in_context_for::presentation_layer::service::request_handler::application_programming_interface::version_1::mobile::_in_context_for::domain_layer::entity::application_user::_new_for_context::authorization::_new_for_context::log_out_::base::Base as ResponseData;
use crate::presentation_layer::service::request_handler::application_programming_interface::version_1::mobile::_in_context_for::domain_layer::entity::application_user::_new_for_context::authorization::Authorization;
use http::StatusCode;
use hyper::body::to_bytes;

pub struct Base;

impl Base {
    pub async fn handle(
        redis_connection_pool: Pool<RedisConnectionManager>,
        request_data: RequestData
    ) -> Result<ResponseData, BaseError> {
        let response = Authorization::log_out(request_data.into_inner(), redis_connection_pool).await;

        let response_data: ResponseData;

        let (
            response_parts,
            body
        ) = response.into_parts();

        if response_parts.status == StatusCode::OK {
            let bytes = to_bytes(body).await?;

            let wrapped_response = rmp_serde::from_read_ref::<'_, [u8], WrappedResponseData<()>>(bytes.chunk())?;

            response_data = ResponseData::new(response_parts, Some(wrapped_response));
        } else {
            response_data = ResponseData::new(response_parts, None);
        }

        return Ok(response_data);
    }
}