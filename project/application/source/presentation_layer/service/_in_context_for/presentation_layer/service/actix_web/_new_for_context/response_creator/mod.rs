use actix_http::body::BoxBody;
use actix_web::http::header;
use actix_web::HttpResponse;
use crate::infrastructure_layer::error::base_error::base_error::BaseError;
use http::Extensions;
use http::HeaderMap;
use http::HeaderValue;
use http::response::Parts;
use http::Version;
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
    ) -> Response<Body> {
        let mut header_map = HeaderMap::new();
        header_map.append(header::CONTENT_TYPE, HeaderValue::from_static("application/octet-stream"));
        header_map.append(header::X_CONTENT_TYPE_OPTIONS, HeaderValue::from_static("nosniff"));

        let mut parts = Response::new(()).into_parts().0;
        parts.status = status_code;
        parts.version = Version::HTTP_2;
        parts.headers = header_map;
        
        let body: Body;
        match data {
            Some(data_) => {
                body = Body::from(data_);
            },
            None => {
                body = Body::empty()
            }
        }

        return Response::from_parts(parts, body);
    }

    pub fn create_bad_request(
    ) -> Response<Body> {
        return Self::create(StatusCode::BAD_REQUEST, None);
    }

    pub fn create_unauthorized(
    ) -> Response<Body> {
        return Self::create(StatusCode::UNAUTHORIZED, None);
    }

    pub fn create_internal_server_error(
    ) -> Response<Body> {
        return Self::create(StatusCode::INTERNAL_SERVER_ERROR, None);
    }

    pub fn create_ok(
        data: Vec<u8>
    ) -> Response<Body> {
        return Self::create(StatusCode::OK, Some(data));
    }

    #[cfg(feature="facilitate_non_automatic_functional_testing")]
    pub fn create_with_status_code(
        status_code: StatusCode,
        data: Option<Vec<u8>>
    ) -> Response<Body> {
        return Self::create(status_code, data);
    }
}