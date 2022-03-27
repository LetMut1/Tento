use bb8_redis::RedisConnectionManager;
use bb8::Pool;
use bytes::Buf;
use crate::infrastructure_layer::error::base_error::base_error::BaseError;
use crate::presentation_layer::data_transfer_object::_in_context_for::presentation_layer::service::controller::_new_for_context::endpoint_response::endpoint_response::EndpointResponse;
use crate::presentation_layer::data_transfer_object::request_data::_in_context_for::presentation_layer::service::controller::version_1::mobile::_in_context_for::domain_layer::entity::application_user::_new_for_context::authorization::_new_for_context::log_in_by_last_step_::base::Base as RequestData;
use crate::presentation_layer::data_transfer_object::response_data::_in_context_for::presentation_layer::service::controller::version_1::mobile::_in_context_for::domain_layer::entity::application_user::_new_for_context::authorization::_new_for_context::log_in_by_last_step_::base::Base as ResponseData;
use crate::presentation_layer::data_transfer_object::response_data::_in_context_for::presentation_layer::service::controller::version_1::mobile::_in_context_for::domain_layer::entity::application_user::_new_for_context::authorization::_new_for_context::log_in_by_last_step::base::Base as ResponseDataLogInByLastStep;
use crate::presentation_layer::service::controller::version_1::mobile::_in_context_for::domain_layer::entity::application_user::_new_for_context::authorization::Authorization;
use http::StatusCode;
use hyper::Body;
use hyper::body::to_bytes;
use hyper::Request;
use std::convert::From;

pub struct Base;

impl Base {
    pub async fn handle(
        redis_connection_pool: Pool<RedisConnectionManager>,
        request_data: RequestData
    ) -> Result<ResponseData, BaseError> {
        let (
            request_parts,
            convertible_data
        ) = request_data.into_inner();

        let mut data: Vec<u8> = vec![];
        rmp_serde::encode::write(&mut data, &convertible_data)?;

        let request = Request::from_parts(request_parts, Body::from(data));
        
        let response = Authorization::log_in_by_last_step(request, redis_connection_pool).await;

        let response_data: ResponseData;

        let (
            response_parts,
            body
        ) = response.into_parts();

        if response_parts.status == StatusCode::OK {
            let bytes = to_bytes(body).await?;

            let endpoint_response = rmp_serde::from_read_ref::<'_, [u8], EndpointResponse<ResponseDataLogInByLastStep>>(bytes.chunk())?;

            response_data = ResponseData::new(response_parts, Some(endpoint_response));
        } else {
            response_data = ResponseData::new(response_parts, None);
        }

        return Ok(response_data);
    }
}