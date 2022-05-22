use crate::application_layer::data_transfer_object::request_data::_in_context_for::application_layer::service::action_handler::_in_context_for::presentation_layer::service::controller::mobile::version_1::_in_context_for::domain_layer::entity::application_user::_new_for_context::authorization::_new_for_context::register_by_last_step::base::_new_for_context::base::Base as RequestData;
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