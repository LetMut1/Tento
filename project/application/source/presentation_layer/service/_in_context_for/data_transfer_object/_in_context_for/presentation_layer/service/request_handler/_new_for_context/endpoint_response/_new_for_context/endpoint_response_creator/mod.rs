use crate::presentation_layer::data_transfer_object::_in_context_for::presentation_layer::service::request_handler::_new_for_context::endpoint_response::endpoint_response::EndpointResponse;
use serde::Serialize;

pub struct EndpointResponseCreator;

impl EndpointResponseCreator {
    pub fn create_without_data(
    ) -> EndpointResponse<()> {
        return EndpointResponse::new_without_data();
    }

    pub fn create_with_data<S>(
        data: S
    ) -> EndpointResponse<S>
    where
        S: Serialize
    {
        return EndpointResponse::new_with_data(data);
    }

    #[cfg(not(feature="facilitate_non_automatic_functional_testing"))]
    pub fn create_with_error_code(
        error_code: &'static str
    ) -> EndpointResponse<()> {
        return EndpointResponse::new_with_error_code(error_code)
    }

    #[cfg(feature="facilitate_non_automatic_functional_testing")]
    pub fn create_with_error_code(
        error_code: &'static str
    ) -> EndpointResponse<()> {
        return EndpointResponse::new_with_error_code(error_code.to_string());
    }
}