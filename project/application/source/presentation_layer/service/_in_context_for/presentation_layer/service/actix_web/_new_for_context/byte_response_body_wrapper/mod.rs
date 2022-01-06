use crate::presentation_layer::data_transfer_object::_in_context_for::presentation_layer::service::_in_context_for::presentation_layer::service::actix_web::_new_for_context::standard_json_response_body_wrapper::_new_for_context::fail_result_with_code::FailResultWithCode;
use crate::presentation_layer::data_transfer_object::_in_context_for::presentation_layer::service::_in_context_for::presentation_layer::service::actix_web::_new_for_context::standard_json_response_body_wrapper::_new_for_context::success_result_with_body::SuccessResultWithBody;
use crate::presentation_layer::data_transfer_object::_in_context_for::presentation_layer::service::_in_context_for::presentation_layer::service::actix_web::_new_for_context::standard_json_response_body_wrapper::_new_for_context::success_result::SuccessResult;
use crate::infrastructure_layer::error::base_error::base_error::BaseError;
use serde::Serialize;

pub struct ByteResponseBodyWrapper;

impl ByteResponseBodyWrapper {
    const SUCCESS_RESULT: SuccessResult = SuccessResult::new();

    pub fn wrap_for_success(
    ) -> Result<Vec<u8>, BaseError> {
        return Ok(rmp_serde::to_vec(&Self::SUCCESS_RESULT)?);
    }

    pub fn wrap_for_success_with_body<S>(
        body: S
    ) -> Result<Vec<u8>, BaseError>
    where
        S: Serialize
    {
        return Ok(rmp_serde::to_vec(&SuccessResultWithBody::new(body))?);
    }

    pub fn wrap_for_fail_with_code(
        code: &'static str
    ) -> Result<Vec<u8>, BaseError> {
        return Ok(rmp_serde::to_vec(&FailResultWithCode::new(code))?);
    }
}