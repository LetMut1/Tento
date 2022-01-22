use crate::presentation_layer::data_transfer_object::_in_context_for::presentation_layer::service::_in_context_for::presentation_layer::service::actix_web::_new_for_context::response_data_wrapper_trait::_new_for_context::wrapped_result::WrappedResult;
use serde::Serialize;

pub struct ResponseDataWrapper;

impl ResponseDataWrapper {
    pub fn wrap_for_success(
    ) -> WrappedResult<()> {
        return WrappedResult::new(true, None, None);
    }

    pub fn wrap_for_success_with_body<S>(
        data: S
    ) -> WrappedResult<S>
    where
        S: Serialize
    {
        return WrappedResult::new(true, None, Some(data));
    }

    pub fn wrap_for_fail(
        error_code: &'static str
    ) -> WrappedResult<()> {
        return WrappedResult::new(false, Some(error_code), None);
    }
}