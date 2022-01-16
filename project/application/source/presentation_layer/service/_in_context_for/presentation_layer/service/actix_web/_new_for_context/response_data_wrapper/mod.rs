use crate::presentation_layer::data_transfer_object::_in_context_for::presentation_layer::service::_in_context_for::presentation_layer::service::actix_web::_new_for_context::response_body_wrapper_trait::_new_for_context::{success_result::SuccessResult, success_result_with_body::SuccessResultWithBody, fail_result_with_code::FailResultWithCode};
use serde::Serialize;

pub struct ResponseDataWrapper;

impl ResponseDataWrapper {                                          // TODO Все три метода должны отдавать одну и ту же структуру. СДелать структуру под трислуча.
    const SUCCESS_RESULT: SuccessResult = SuccessResult::new();

    pub fn wrap_for_success(
    ) -> SuccessResult {
        return Self::SUCCESS_RESULT;
    }

    pub fn wrap_for_success_with_body<S>(
        data: S
    ) -> SuccessResultWithBody<S>
    where
        S: Serialize
    {
        return SuccessResultWithBody::new(data);
    }

    pub fn wrap_for_fail(
        code: &'static str
    ) -> FailResultWithCode {
        return FailResultWithCode::new(code);
    }
}