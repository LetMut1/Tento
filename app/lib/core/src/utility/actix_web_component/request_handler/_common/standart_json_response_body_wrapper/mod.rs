use crate::dto::utility::actix_web_component::request_handler::_common::standart_response_body_wrapper::fail_result::FailResult;
use crate::dto::utility::actix_web_component::request_handler::_common::standart_response_body_wrapper::success_result::SuccessResult;
use serde_json;
use serde::Serialize;

pub struct StandartJsonResponseBodyWrapper;

impl<'outer> StandartJsonResponseBodyWrapper {
    pub fn create_for_success<S>(body: &'outer S) -> String 
    where 
        S: Serialize
    {
        return serde_json::to_string::<SuccessResult<S>>(&SuccessResult::new(body)).unwrap();    // TODO обработать
    }

    pub fn create_for_fail(code: &'static str) -> String {
        return serde_json::to_string::<FailResult>(&FailResult::new(code)).unwrap();        // TODO Обработать
    }
}