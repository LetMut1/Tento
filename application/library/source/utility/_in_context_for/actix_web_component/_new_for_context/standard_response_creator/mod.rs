use actix_web::dev::Body;
use actix_web::http::header;
use actix_web::HttpResponse;
use crate::error::main_error::main_error::MainError;
use serde::Serialize;
use super::standard_json_response_body_wrapper::StandardJsonResponseBodyWrapper;

pub struct StandardResponseCreator;

impl StandardResponseCreator {
    pub fn wrap_for_success_and_create_ok() -> HttpResponse<Body> {
        let success_response_body: String = match StandardJsonResponseBodyWrapper::wrap_for_success() {
            Ok(success_response_body) => success_response_body,
            Err(ref main_error) => {
                match main_error {
                    MainError::RunTimeError(_) => {
                        log::error!("{}", main_error);

                        return StandardResponseCreator::create_internal_server_error();
                    },
                    _ => {
                        unreachable!("{}", main_error);
                    }
                }

            }
        };

        return Self::create_ok(success_response_body);
    }

    pub fn wrap_for_success_with_body_and_create_ok<'outer_a, S>(body: &'outer_a S) -> HttpResponse<Body>
    where 
        S: Serialize
    {
        let success_with_body_response_body: String = match StandardJsonResponseBodyWrapper::wrap_for_success_with_body(body) {
            Ok(success_with_body_response_body) => success_with_body_response_body,
            Err(ref main_error) => {
                match main_error {
                    MainError::RunTimeError(_) => {
                        log::error!("{}", main_error);

                        return StandardResponseCreator::create_internal_server_error();
                    },
                    _ => {
                        unreachable!("{}", main_error);
                    }
                }

            }
        };

        return Self::create_ok(success_with_body_response_body);
    }

    pub fn wrap_for_fail_with_code_and_create_ok(code: &'static str) -> HttpResponse<Body> {
        let fail_with_code_response_body: String = match StandardJsonResponseBodyWrapper::wrap_for_fail_with_code(code) {
            Ok(fail_with_code_response_body) => fail_with_code_response_body,
            Err(ref main_error) => {
                match main_error {
                    MainError::RunTimeError(_) => {
                        log::error!("{}", main_error);

                        return StandardResponseCreator::create_internal_server_error();
                    },
                    _ => {
                        unreachable!("{}", main_error);
                    }
                }

            }
        };

        return Self::create_ok(fail_with_code_response_body);
    }

    pub fn create_bad_request() -> HttpResponse<Body> {
        return HttpResponse::BadRequest().finish();
    }

    pub fn create_internal_server_error() -> HttpResponse<Body> {
        return HttpResponse::InternalServerError().finish();
    }

    fn create_ok(body: String) -> HttpResponse<Body> {
        return HttpResponse::Ok()
        .set_header(header::CONTENT_TYPE, "application/json")
        .body(body);
    }
}