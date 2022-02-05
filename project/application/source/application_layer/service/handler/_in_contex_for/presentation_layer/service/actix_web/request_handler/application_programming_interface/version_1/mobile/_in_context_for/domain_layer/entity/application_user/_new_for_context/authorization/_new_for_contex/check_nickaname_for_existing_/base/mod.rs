use actix_http::body::BoxBody;
use actix_http::StatusCode;
use actix_web::body;
use actix_web::FromRequest;
use actix_web::HttpRequest;
use actix_web::HttpResponse;
use actix_web::web::Buf;
use actix_web::web::Bytes;
use actix_web::web::Payload;
use crate::infrastructure_layer::error::base_error::base_error::BaseError;
use crate::infrastructure_layer::service::http_payload_creator::HttpPayloadCreator;
use crate::presentation_layer::data_transfer_object::_in_context_for::presentation_layer::service::_in_context_for::presentation_layer::service::actix_web::_new_for_context::response_data_wrapper_trait::_new_for_context::wrapped_response::WrappedResponse;
use crate::presentation_layer::data_transfer_object::request::_in_context_for::presentation_layer::service::actix_web::request_handler::application_programming_interface::version_1::mobile::_in_context_for::domain_layer::entity::application_user::_new_for_context::authorization::_new_for_context::check_nickname_for_existing::base::Base as Request;
use crate::presentation_layer::data_transfer_object::response::_in_context_for::presentation_layer::service::actix_web::request_handler::application_programming_interface::version_1::mobile::_in_context_for::domain_layer::entity::application_user::_new_for_context::authorization::_new_for_context::check_nickname_for_existing::base::Base as Response;
use crate::presentation_layer::service::actix_web::request_handler::application_programming_interface::version_1::mobile::_in_context_for::domain_layer::entity::application_user::_new_for_context::authorization::Authorization;
use std::convert::From;

pub struct Base;

impl Base {
    pub async fn handle(
        http_request: HttpRequest,
        request: Request
    ) -> Result<(Option<WrappedResponse<Response>>, StatusCode), BaseError> {
        let mut data: Vec<u8> = vec![];
        rmp_serde::encode::write(&mut data, &request)?;

        let payload: Payload = Payload::from_request(
            &http_request, &mut HttpPayloadCreator::create_from_data(Bytes::from(data))
        ).await.unwrap();     // TODO resolve unwrap !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

        let http_response: HttpResponse<BoxBody> = Authorization::check_nickname_for_existing(http_request, payload).await;

        let status_code: StatusCode = http_response.status();

        let mut result: (Option<WrappedResponse<Response>>, StatusCode) = (None, status_code);

        if status_code == StatusCode::OK {
            let body: BoxBody = http_response.into_parts().1;

            let mut bytes: Bytes = body::to_bytes(body).await.unwrap();    // TODO resolve unwrap !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

            let mut data: Vec<u8> = vec![];
            bytes.copy_to_slice(&mut data);

            let wrapped_response: WrappedResponse<Response> = rmp_serde::from_read_ref::<'_, [u8], WrappedResponse<Response>>(&data[..])?;

            result = (Some(wrapped_response), status_code);
        }

        return Ok(result);
    }
}