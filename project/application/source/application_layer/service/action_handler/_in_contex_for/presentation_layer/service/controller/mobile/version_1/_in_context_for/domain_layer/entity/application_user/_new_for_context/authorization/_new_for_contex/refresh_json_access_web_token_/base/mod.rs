use bb8_redis::RedisConnectionManager;
use bb8::Pool;
use bytes::Buf;
use crate::infrastructure_layer::error::base_error::base_error::BaseError;
use crate::presentation_layer::data_transfer_object::_in_context_for::presentation_layer::service::controller::_new_for_context::endpoint_response::endpoint_response::EndpointResponse;
use crate::presentation_layer::data_transfer_object::request_data::_in_context_for::presentation_layer::service::controller::mobile::version_1::_in_context_for::domain_layer::entity::application_user::_new_for_context::authorization::_new_for_context::refresh_json_access_web_token_::base::Base as RequestData;
use crate::presentation_layer::data_transfer_object::response_data::_in_context_for::presentation_layer::service::controller::mobile::version_1::_in_context_for::domain_layer::entity::application_user::_new_for_context::authorization::_new_for_context::refresh_json_access_web_token_::base::Base as ResponseData;
use crate::presentation_layer::data_transfer_object::response_data::_in_context_for::presentation_layer::service::controller::mobile::version_1::_in_context_for::domain_layer::entity::application_user::_new_for_context::authorization::_new_for_context::refresh_json_access_web_token::base::Base as ResponseDataRefreshJsonAccessWebToken;
use crate::presentation_layer::service::controller::mobile::version_1::_in_context_for::domain_layer::entity::application_user::_new_for_context::authorization::Authorization;
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
        
        let response = Authorization::refresh_json_access_web_token(request, redis_connection_pool).await;

        let response_data: ResponseData;

        let (
            response_parts,
            body
        ) = response.into_parts();

        if response_parts.status == StatusCode::OK {
            let bytes = to_bytes(body).await?;

            let endpoint_response = rmp_serde::from_read_ref::<'_, [u8], EndpointResponse<ResponseDataRefreshJsonAccessWebToken>>(bytes.chunk())?;

            response_data = ResponseData::new(response_parts, Some(endpoint_response));
        } else {
            response_data = ResponseData::new(response_parts, None);
        }

        return Ok(response_data);
    }
}