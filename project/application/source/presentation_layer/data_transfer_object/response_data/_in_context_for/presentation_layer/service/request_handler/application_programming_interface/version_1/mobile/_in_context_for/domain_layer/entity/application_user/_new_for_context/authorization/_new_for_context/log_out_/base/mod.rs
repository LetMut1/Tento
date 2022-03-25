use crate::presentation_layer::data_transfer_object::_in_context_for::presentation_layer::service::response_data_wrapper::_new_for_context::wrapped_response::WrappedResponseData;
use http::response::Parts;

pub struct Base {
    parts: Parts,
    convertible_data: Option<WrappedResponseData<()>>
}

impl Base {
    pub fn new(
        parts: Parts,
        convertible_data: Option<WrappedResponseData<()>>
    ) -> Self {
        return Self {
            parts,
            convertible_data
        };
    }

    pub fn into_inner(
        self
    ) -> (Parts, Option<WrappedResponseData<()>>) {
        return (
            self.parts,
            self.convertible_data,
        );
    }
}