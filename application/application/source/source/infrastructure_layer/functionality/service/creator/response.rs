use super::Creator;
use crate::infrastructure_layer::data::control_type::Response;
use http::{
    header,HeaderMap, HeaderValue, StatusCode, Version
};
use hyper::Response as HyperResponse;
use http_body_util::Full;
use bytes::Bytes;
use std::convert::From;
impl Creator<Response> {
    pub const HEADER_VALUE_CONTENT_TYPE: HeaderValue = HeaderValue::from_static("application/octet-stream");
    pub const HEADER_VALUE_X_CONTENT_TYPE_OPTIONS: HeaderValue = HeaderValue::from_static("nosniff");
    fn create(status_code: StatusCode, data: Option<Vec<u8>>) -> Response {
        let mut header_map = HeaderMap::new();                      // TODO TODO TODO TODO TODO  ContentLength is needed?
        header_map.append(
            header::CONTENT_TYPE,
            Self::HEADER_VALUE_CONTENT_TYPE,
        );
        header_map.append(
            header::X_CONTENT_TYPE_OPTIONS,
            Self::HEADER_VALUE_X_CONTENT_TYPE_OPTIONS,
        );
        let mut parts = HyperResponse::new(()).into_parts().0;
        parts.status = status_code;
        parts.version = Version::HTTP_2;
        parts.headers = header_map;
        let bytes = match data {
            Some(data_) => Bytes::from(data_),
            None => Bytes::new(),
        };
        return Response::from_parts(
            parts,
            Full::new(bytes),
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