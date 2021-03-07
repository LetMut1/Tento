use crate::dto::_in_context_for::utility::_in_context_for::actix_web_component::request_handler::_new_for_context::standart_json_response_body_wrapper::_new_for_context::fail_result::FailResult;
use crate::dto::_in_context_for::utility::_in_context_for::actix_web_component::request_handler::_new_for_context::standart_json_response_body_wrapper::_new_for_context::success_result::SuccessResult;
use serde_json;
use serde::Serialize;

pub struct StandartJsonResponseBodyWrapper;

impl<'outer> StandartJsonResponseBodyWrapper {
    pub fn wrap_for_success<S>(body: &'outer S) -> String 
    where 
        S: Serialize
    {
        return serde_json::to_string::<SuccessResult<S>>(&SuccessResult::new(body)).unwrap();    // TODO обработать
    }

    pub fn wrap_for_fail(code: &'static str) -> String {
        return serde_json::to_string::<FailResult>(&FailResult::new(code)).unwrap();        // TODO Обработать LogikError это не должно выбрасываться 
    }
}