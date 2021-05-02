use crate::data_transfer_object::_in_context_for::utility::_in_context_for::actix_web_component::request_handler::_new_for_context::standard_json_response_body_wrapper::_new_for_context::fail_result_with_code::FailResultWithCode;
use crate::data_transfer_object::_in_context_for::utility::_in_context_for::actix_web_component::request_handler::_new_for_context::standard_json_response_body_wrapper::_new_for_context::success_result_with_body::SuccessResultWithBody;
use crate::data_transfer_object::_in_context_for::utility::_in_context_for::actix_web_component::request_handler::_new_for_context::standard_json_response_body_wrapper::_new_for_context::success_result::SuccessResult;
use serde_json;
use serde::Serialize;

pub struct StandardJsonResponseBodyWrapper;

impl<'outer_a> StandardJsonResponseBodyWrapper {
    pub fn wrap_for_success_with_body<S>(body: &'outer_a S) -> String 
    where 
        S: Serialize
    {
        return serde_json::to_string(&SuccessResultWithBody::new(body)).unwrap();    // TODO обработать
    }

    pub fn wrap_for_success() -> String {
        return serde_json::to_string(&SuccessResult::new()).unwrap();    // TODO обработать
    }

    pub fn wrap_for_fail_with_code(code: &'static str) -> String {
        return serde_json::to_string(&FailResultWithCode::new(code)).unwrap();        // TODO Обработать LogikError это не должно выбрасываться 
    }
}