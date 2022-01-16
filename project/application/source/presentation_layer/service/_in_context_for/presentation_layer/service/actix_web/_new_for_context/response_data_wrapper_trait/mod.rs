use crate::presentation_layer::data_transfer_object::_in_context_for::presentation_layer::service::_in_context_for::presentation_layer::service::actix_web::_new_for_context::response_body_wrapper_trait::_new_for_context::success_result::SuccessResult;
use serde::Serialize;
use std::error::Error;

pub trait ResponseDataWrapperTrait {
    const SUCCESS_RESULT: SuccessResult = SuccessResult::new();

    type WrappedDataType;
    type Error: Error;

    fn wrap_for_success(
    ) -> Result<Self::WrappedDataType, Self::Error>;

    fn wrap_for_success_with_body<S>(
        body: S
    ) -> Result<Self::WrappedDataType, Self::Error>
    where
        S: Serialize;

    fn wrap_for_fail(
        code: &'static str
    ) -> Result<Self::WrappedDataType, Self::Error>;
}