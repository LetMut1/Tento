use actix_web::dev::Body;
use actix_web::http::header;
use actix_web::HttpResponse;
use crate::infrastructure_layer::error::base_error::base_error::BaseError;
use serde::Serialize;
use super::standard_json_response_body_wrapper::StandardJsonResponseBodyWrapper;

pub struct StandardResponseCreator;

impl StandardResponseCreator {
    pub fn wrap_for_success_and_create_ok(
    ) -> HttpResponse<Body> {
        match StandardJsonResponseBodyWrapper::wrap_for_success() {
            Ok(success_response_body) => {
                return Self::create_ok(success_response_body);
            },
            Err(ref base_error) => {
                match base_error {
                    BaseError::RunTimeError {run_time_error: _} => {
                        log::error!("{}", base_error);

                        return StandardResponseCreator::create_internal_server_error();
                    },
                    _ => {
                        unreachable!("{}", base_error);
                    }
                }

            }
        }
    }

    pub fn wrap_for_success_with_body_and_create_ok<'a, S>(
        body: &'a S
    ) -> HttpResponse<Body>
    where
        S: Serialize
    {
        match StandardJsonResponseBodyWrapper::wrap_for_success_with_body(body) {
            Ok(success_with_body_response_body) => {
                return Self::create_ok(success_with_body_response_body);
            },
            Err(ref base_error) => {
                match base_error {
                    BaseError::RunTimeError {run_time_error: _} => {
                        log::error!("{}", base_error);

                        return StandardResponseCreator::create_internal_server_error();
                    },
                    _ => {
                        unreachable!("{}", base_error);
                    }
                }

            }
        }
    }

    pub fn wrap_for_fail_with_code_and_create_ok(
        code: &'static str
    ) -> HttpResponse<Body> {
        match StandardJsonResponseBodyWrapper::wrap_for_fail_with_code(code) {
            Ok(fail_with_code_response_body) => {
                return Self::create_ok(fail_with_code_response_body);
            },
            Err(ref base_error) => {
                match base_error {
                    BaseError::RunTimeError {run_time_error: _} => {
                        log::error!("{}", base_error);

                        return StandardResponseCreator::create_internal_server_error();
                    },
                    _ => {
                        unreachable!("{}", base_error);
                    }
                }

            }
        }
    }

    pub fn create_bad_request(
    ) -> HttpResponse<Body> {
        return HttpResponse::BadRequest().finish();
    }

    pub fn create_unauthorized(
    ) -> HttpResponse<Body> {
        return HttpResponse::Unauthorized().finish();
    }

    pub fn create_internal_server_error(
    ) -> HttpResponse<Body> {
        return HttpResponse::InternalServerError().finish();
    }

    fn create_ok(
        body: String
    ) -> HttpResponse<Body> {
        return HttpResponse::Ok()
        .set_header(header::CONTENT_TYPE, "application/json")
        .body(body);
    }
}