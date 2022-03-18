use crate::presentation_layer::data_transfer_object::request_data::_in_context_for::presentation_layer::service::request_handler::application_programming_interface::version_1::mobile::_in_context_for::domain_layer::entity::application_user::_new_for_context::authorization::_new_for_context::send_email_for_log_in::base::Base as RequestData;
use http::request::Parts;

pub struct Base {
    parts: Parts,
    convertible_data: RequestData
}

impl Base {
    pub fn new(
        parts: Parts,
        convertible_data: RequestData
    ) -> Self {
        return Self {
            parts,
            convertible_data
        };
    }

    pub fn into_inner(
        self
    ) -> (Parts, RequestData) {
        return (
            self.parts,
            self.convertible_data
        );
    }
}