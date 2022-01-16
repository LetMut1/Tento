use crate::infrastructure_layer::error::base_error::base_error::BaseError;
use crate::presentation_layer::data_transfer_object::_in_context_for::presentation_layer::service::_in_context_for::presentation_layer::service::actix_web::_new_for_context::response_body_wrapper_trait::_new_for_context::fail_result_with_code::FailResultWithCode;
use crate::presentation_layer::data_transfer_object::_in_context_for::presentation_layer::service::_in_context_for::presentation_layer::service::actix_web::_new_for_context::response_body_wrapper_trait::_new_for_context::success_result_with_body::SuccessResultWithBody;
use serde::Serialize;
use super::response_data_wrapper_trait::ResponseDataWrapperTrait;

pub struct JsonResponseDataWrapper;

impl ResponseDataWrapperTrait for  JsonResponseDataWrapper {
    type WrappedDataType = String;
    type Error = BaseError;

    fn wrap_for_success(
    ) -> Result<Self::WrappedDataType, Self::Error> {
        return Ok(serde_json::to_string(&Self::SUCCESS_RESULT)?);
    }

    fn wrap_for_success_with_body<S>(
        body: S
    ) -> Result<Self::WrappedDataType, Self::Error>
    where
        S: Serialize
    {
        return Ok(serde_json::to_string(&SuccessResultWithBody::new(body))?);
    }

    fn wrap_for_fail(
        code: &'static str
    ) -> Result<Self::WrappedDataType, Self::Error> {
        return Ok(serde_json::to_string(&FailResultWithCode::new(code))?);
    }
}