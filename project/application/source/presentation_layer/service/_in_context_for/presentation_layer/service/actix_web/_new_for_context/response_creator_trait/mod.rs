use actix_web::body::Body;
use actix_web::HttpResponse;
use serde::Serialize;
use super::response_data_wrapper_trait::ResponseDataWrapperTrait;

pub trait ResponseCreatorTrait: ResponseCreatorTrait_ {
    fn wrap_for_success_and_create_ok(
    ) -> Result<HttpResponse<Body>, <Self::ResponseDataWrapper as ResponseDataWrapperTrait>::Error> {
        return Ok(Self::create_ok(Self::ResponseDataWrapper::wrap_for_success()?));
    }

    fn wrap_for_success_with_body_and_create_ok<S>(
        body: S
    ) -> Result<HttpResponse<Body>, <Self::ResponseDataWrapper as ResponseDataWrapperTrait>::Error>
    where
        S: Serialize
    {
        return Ok(Self::create_ok(Self::ResponseDataWrapper::wrap_for_success_with_body(body)?));
    }

    fn wrap_for_fail_and_create_ok(
        code: &'static str
    ) -> Result<HttpResponse<Body>, <Self::ResponseDataWrapper as ResponseDataWrapperTrait>::Error> {
        return Ok(Self::create_ok(Self::ResponseDataWrapper::wrap_for_fail(code)?));
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
    type ResponseDataWrapper: ResponseDataWrapperTrait;                 // TODO ВСЕ ЧЕРЕЗ БАЙТЫ!!!!!!!!!!!!!!!

    fn create_ok(
        body: <Self::ResponseDataWrapper as ResponseDataWrapperTrait>::WrappedDataType
    ) -> HttpResponse<Body>;
}