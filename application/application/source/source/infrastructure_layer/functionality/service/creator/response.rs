use super::Creator;
use crate::infrastructure_layer::data::control_type::Response;
#[cfg(feature = "manual_testing")]
use http::response::Parts;
use http::{
    header,
    HeaderMap,
    HeaderValue,
    Version,
};
use hyper::{
    Body,
    Response as HyperResponse,
    StatusCode,
};
use std::convert::From;
impl Creator<Response> {
    pub const HEADER_VALUE_CONTENT_TYPE: &'static str = "application/octet-stream";
    // TODO В файл с константами
    pub const HEADER_VALUE_X_CONTENT_TYPE_OPTIONS: &'static str = "nosniff";
    // TODO В файл с константами
    fn create(status_code: StatusCode, data: Option<Vec<u8>>) -> Response {
        // TODO Посмотреть, что за дефолтные ответ. НАстроить необходимое
        let mut header_map = HeaderMap::new();
        header_map.append(
            header::CONTENT_TYPE,
            HeaderValue::from_static(Self::HEADER_VALUE_CONTENT_TYPE),
        );
        header_map.append(
            header::X_CONTENT_TYPE_OPTIONS,
            HeaderValue::from_static(Self::HEADER_VALUE_X_CONTENT_TYPE_OPTIONS),
        );
        let mut parts = HyperResponse::new(()).into_parts().0;
        parts.status = status_code;
        parts.version = Version::HTTP_2;
        parts.headers = header_map;
        let body = match data {
            Some(data_) => Body::from(data_),
            None => Body::empty(),
        };
        return Response::from_parts(
            parts,
            body,
        );
    }
    pub fn create_bad_request() -> Response {
        return Self::create(
            StatusCode::BAD_REQUEST,
            None,
        );
    }
    pub fn create_unauthorized() -> Response {
        return Self::create(
            StatusCode::UNAUTHORIZED,
            None,
        );
    }
    pub fn create_not_found() -> Response {
        return Self::create(
            StatusCode::NOT_FOUND,
            None,
        );
    }
    pub fn create_internal_server_error() -> Response {
        return Self::create(
            StatusCode::INTERNAL_SERVER_ERROR,
            None,
        );
    }
    pub fn create_not_extended() -> Response {
        return Self::create(
            StatusCode::NOT_EXTENDED,
            None,
        );
    }
    pub fn create_ok(data: Vec<u8>) -> Response {
        return Self::create(
            StatusCode::OK,
            Some(data),
        );
    }
}
#[cfg(feature = "manual_testing")]
impl Creator<Response> {
    pub fn create_from_response_parts(response_parts: Parts, data: Option<Vec<u8>>) -> Response {
        match data {
            Some(data_) => {
                return Response::from_parts(
                    response_parts,
                    Body::from(data_),
                );
            }
            None => {
                return Response::from_parts(
                    response_parts,
                    Body::empty(),
                );
            }
        }
    }
}
