use crate::presentation_layer::data_transfer_object::_in_context_for::presentation_layer::service::_in_context_for::presentation_layer::service::actix_web_component::_new_for_context::standard_json_response_body_wrapper::_new_for_context::fail_result_with_code::FailResultWithCode;
use crate::presentation_layer::data_transfer_object::_in_context_for::presentation_layer::service::_in_context_for::presentation_layer::service::actix_web_component::_new_for_context::standard_json_response_body_wrapper::_new_for_context::success_result_with_body::SuccessResultWithBody;
use crate::presentation_layer::data_transfer_object::_in_context_for::presentation_layer::service::_in_context_for::presentation_layer::service::actix_web_component::_new_for_context::standard_json_response_body_wrapper::_new_for_context::success_result::SuccessResult;
use crate::domain_layer::error::base_error::base_error::BaseError;
use serde::Serialize;

pub struct StandardJsonResponseBodyWrapper;

impl StandardJsonResponseBodyWrapper {
    const SUCCESS_RESULT: &'static SuccessResult = &SuccessResult::new();

    pub fn wrap_for_success() -> Result<String, BaseError> {
        return Ok(serde_json::to_string(Self::SUCCESS_RESULT)?);
    }

    pub fn wrap_for_success_with_body<'outer_a, S>(body: &'outer_a S) -> Result<String, BaseError>
    where 
        S: Serialize
    {
        return Ok(serde_json::to_string(&SuccessResultWithBody::new(body))?);
    }

    pub fn wrap_for_fail_with_code(code: &'static str) -> Result<String, BaseError> {
        return Ok(serde_json::to_string(&FailResultWithCode::new(code))?);
    }
}