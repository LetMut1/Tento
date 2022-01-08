use crate::presentation_layer::data_transfer_object::_in_context_for::presentation_layer::service::_in_context_for::presentation_layer::service::actix_web::_new_for_context::response_body_wrapper_trait::_new_for_context::success_result::SuccessResult;
use serde::Serialize;
use std::error::Error;

pub trait ResponseBodyWrapperTrait {
    const SUCCESS_RESULT: SuccessResult = SuccessResult::new();

    type WrappedBodyType;
    type Error: Error;

    fn wrap_for_success(
    ) -> Result<Self::WrappedBodyType, Self::Error>;

    fn wrap_for_success_with_body<S>(
        body: S
    ) -> Result<Self::WrappedBodyType, Self::Error>
    where
        S: Serialize;

    fn wrap_for_fail(
        code: &'static str
    ) -> Result<Self::WrappedBodyType, Self::Error>;
}