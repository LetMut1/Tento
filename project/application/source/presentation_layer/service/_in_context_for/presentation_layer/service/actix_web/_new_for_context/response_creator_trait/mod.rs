use actix_web::body::Body;
use actix_web::HttpResponse;
use serde::Serialize;
use super::response_body_wrapper_trait::ResponseBodyWrapperTrait;

pub trait ResponseCreatorTrait: ResponseCreatorTrait_ {
    fn wrap_for_success_and_create_ok(
    ) -> Result<HttpResponse<Body>, <Self::ResponseBodyWrapper as ResponseBodyWrapperTrait>::Error> {
        return Ok(Self::create_ok(Self::ResponseBodyWrapper::wrap_for_success()?));
    }

    fn wrap_for_success_with_body_and_create_ok<S>(
        body: S
    ) -> Result<HttpResponse<Body>, <Self::ResponseBodyWrapper as ResponseBodyWrapperTrait>::Error>
    where
        S: Serialize
    {
        return Ok(Self::create_ok(Self::ResponseBodyWrapper::wrap_for_success_with_body(body)?));
    }

    fn wrap_for_fail_and_create_ok(
        code: &'static str
    ) -> Result<HttpResponse<Body>, <Self::ResponseBodyWrapper as ResponseBodyWrapperTrait>::Error> {
        return Ok(Self::create_ok(Self::ResponseBodyWrapper::wrap_for_fail(code)?));
    }

    fn create_bad_request(
    ) -> HttpResponse<Body> {
        return HttpResponse::BadRequest().finish();
    }

    fn create_unauthorized(
    ) -> HttpResponse<Body> {
        return HttpResponse::Unauthorized().finish();
    }

    fn create_internal_server_error(
    ) -> HttpResponse<Body> {
        return HttpResponse::InternalServerError().finish();
    }
}

pub trait ResponseCreatorTrait_ {
    type ResponseBodyWrapper: ResponseBodyWrapperTrait;

    fn create_ok(
        body: <Self::ResponseBodyWrapper as ResponseBodyWrapperTrait>::WrappedBodyType
    ) -> HttpResponse<Body>;
}