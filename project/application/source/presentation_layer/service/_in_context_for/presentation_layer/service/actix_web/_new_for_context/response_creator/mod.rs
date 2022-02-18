use actix_http::body::BoxBody;
use actix_web::http::header;
use actix_web::HttpResponse;
use crate::infrastructure_layer::error::base_error::base_error::BaseError;
use hyper::Body;
use hyper::Response;
use hyper::StatusCode;
use std::convert::From;

pub struct ResponseCreator;

impl ResponseCreator {
    fn createXXXxDelete(
        status_code: StatusCode,
        data: Option<Vec<u8>>
    ) -> HttpResponse<BoxBody> {
        if let Some(data_) = data {
            return HttpResponse::build(status_code)
                .set_header(header::CONTENT_TYPE, "application/octet-stream")
                .set_header(header::X_CONTENT_TYPE_OPTIONS, "nosniff")
                .body(data_);
        }

        return HttpResponse::build(status_code).finish();
    }

    pub fn create_bad_requestXXXxDelete(
    ) -> HttpResponse<BoxBody> {
        return Self::createXXXxDelete(StatusCode::BAD_REQUEST, None);
    }

    pub fn create_unauthorizedXXXxDelete(
    ) -> HttpResponse<BoxBody> {
        return Self::createXXXxDelete(StatusCode::UNAUTHORIZED, None);
    }

    pub fn create_internal_server_errorXXXxDelete(
    ) -> HttpResponse<BoxBody> {
        return Self::createXXXxDelete(StatusCode::INTERNAL_SERVER_ERROR, None);
    }

    pub fn create_okXXXxDelete(
        data: Vec<u8>
    ) -> HttpResponse<BoxBody> {
        return Self::createXXXxDelete(StatusCode::OK, Some(data));
    }

    #[cfg(feature="facilitate_non_automatic_functional_testing")]
    pub fn create_with_status_codeXXXxDelete(
        status_code: StatusCode,
        data: Option<Vec<u8>>
    ) -> HttpResponse<BoxBody> {
        return Self::createXXXxDelete(status_code, data);
    }

    fn create(                          // TODO Посмотреть, что за дефолтные ответ. НАстроить необходимое
        status_code: StatusCode,
        data: Option<Vec<u8>>
    ) -> Result<Response<Body>, BaseError> {
        let response: Response<Body>;

        match data {
            Some(data_) => {
                response = Response::builder()
                .status(status_code)
                .header(header::CONTENT_TYPE, "application/octet-stream")
                .header(header::X_CONTENT_TYPE_OPTIONS, "nosniff")
                .body(Body::from(data_))?;
            },
            None => {
                response = Response::builder()
                .status(status_code)
                .body(Body::empty())?;
            }
        }

        return Ok(response);
    }

    pub fn create_bad_request(
    ) -> Result<Response<Body>, BaseError> {
        return Self::create(StatusCode::BAD_REQUEST, None);
    }

    pub fn create_unauthorized(
    ) -> Result<Response<Body>, BaseError> {
        return Self::create(StatusCode::UNAUTHORIZED, None);
    }

    pub fn create_internal_server_error(
    ) -> Result<Response<Body>, BaseError> {
        return Self::create(StatusCode::INTERNAL_SERVER_ERROR, None);
    }

    pub fn create_ok(
        data: Vec<u8>
    ) -> Result<Response<Body>, BaseError> {
        return Self::create(StatusCode::OK, Some(data));
    }

    #[cfg(feature="facilitate_non_automatic_functional_testing")]
    pub fn create_with_status_code(
        status_code: StatusCode,
        data: Option<Vec<u8>>
    ) -> Result<Response<Body>, BaseError> {
        return Self::create(status_code, data);
    }
}