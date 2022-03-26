use crate::presentation_layer::data_transfer_object::_in_context_for::presentation_layer::service::request_handler::application_programming_interface::_new_for_context::endpoint_response::endpoint_response::EndpointResponse;
use crate::presentation_layer::data_transfer_object::response_data::_in_context_for::presentation_layer::service::request_handler::application_programming_interface::version_1::mobile::_in_context_for::domain_layer::entity::application_user::_new_for_context::authorization::_new_for_context::refresh_json_access_web_token::base::Base as ResponseData;
use http::response::Parts;

pub struct Base {
    parts: Parts,
    convertible_data: Option<EndpointResponse<ResponseData>>
}

impl Base {
    pub fn new(
        parts: Parts,
        convertible_data: Option<EndpointResponse<ResponseData>>
    ) -> Self {
        return Self {
            parts,
            convertible_data
        };
    }

    pub fn into_inner(
        self
    ) -> (Parts, Option<EndpointResponse<ResponseData>>) {
        return (
            self.parts,
            self.convertible_data,
        );
    }
}