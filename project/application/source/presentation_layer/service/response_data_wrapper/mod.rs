use crate::presentation_layer::data_transfer_object::_in_context_for::presentation_layer::service::response_data_wrapper::_new_for_context::wrapped_response::WrappedResponseData;
use serde::Serialize;

pub struct ResponseDataWrapper;

impl ResponseDataWrapper {
    pub fn wrap_for_success(
    ) -> WrappedResponseData<()> {
        return WrappedResponseData::new(true, None, None);
    }

    pub fn wrap_for_success_with_body<S>(
        data: S
    ) -> WrappedResponseData<S>
    where
        S: Serialize
    {
        return WrappedResponseData::new(true, None, Some(data));
    }

    #[cfg(not(feature="facilitate_non_automatic_functional_testing"))]
    pub fn wrap_for_fail(
        error_code: &'static str
    ) -> WrappedResponseData<()> {
        return WrappedResponseData::new(false, Some(error_code), None);
    }

    #[cfg(feature="facilitate_non_automatic_functional_testing")]
    pub fn wrap_for_fail(
        error_code: &'static str
    ) -> WrappedResponseData<()> {
        return WrappedResponseData::new(false, Some(error_code.to_string()), None);
    }
}