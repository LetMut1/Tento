use crate::presentation_layer::data_transfer_object::_in_context_for::presentation_layer::service::request_handler::_new_for_context::endpoint_response::endpoint_response::EndpointResponse;
use http::response::Parts;

pub struct Base {
    parts: Parts,
    convertible_data: Option<EndpointResponse<()>>
}

impl Base {
    pub fn new(
        parts: Parts,
        convertible_data: Option<EndpointResponse<()>>
    ) -> Self {
        return Self {
            parts,
            convertible_data
        };
    }

    pub fn into_inner(
        self
    ) -> (Parts, Option<EndpointResponse<()>>) {
        return (
            self.parts,
            self.convertible_data,
        );
    }
}