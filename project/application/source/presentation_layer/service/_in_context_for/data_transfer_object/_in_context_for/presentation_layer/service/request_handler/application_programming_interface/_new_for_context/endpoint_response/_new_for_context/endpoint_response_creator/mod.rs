use crate::presentation_layer::data_transfer_object::_in_context_for::presentation_layer::service::request_handler::application_programming_interface::_new_for_context::endpoint_response::endpoint_response::EndpointResponse;
use serde::Serialize;

pub struct ResponseDataWrapper;

impl ResponseDataWrapper {
    pub fn wrap_without_data(
    ) -> EndpointResponse<()> {
        return EndpointResponse::new_without_data();
    }

    pub fn wrap_for_success_with_body<S>(
        data: S
    ) -> EndpointResponse<S>
    where
        S: Serialize
    {
        return EndpointResponse::new_with_data(data);
    }

    #[cfg(not(feature="facilitate_non_automatic_functional_testing"))]
    pub fn wrap_for_fail(
        error_code: &'static str
    ) -> EndpointResponse<()> {
        return EndpointResponse::new_with_error_code(error_code)
    }

    #[cfg(feature="facilitate_non_automatic_functional_testing")]
    pub fn wrap_for_fail(
        error_code: &'static str
    ) -> EndpointResponse<()> {
        return EndpointResponse::new_with_error_code(error_code.to_string());
    }
}