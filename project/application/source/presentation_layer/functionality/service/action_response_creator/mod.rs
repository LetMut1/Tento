use http::header;
use http::HeaderMap;
use http::HeaderValue;
use http::Version;
use hyper::Body;
use hyper::Response;
use hyper::StatusCode;
use std::convert::From;

pub struct ActionResponseCreator;

impl ActionResponseCreator {
    pub const HEADER_VALUE_CONTENT_TYPE: &'static str = "application/octet-stream";        // TODO В файл с константами
    pub const HEADER_VALUE_X_CONTENT_TYPE_OPTIONS: &'static str = "nosniff";        // TODO В файл с константами

    fn create(                          // TODO Посмотреть, что за дефолтные ответ. НАстроить необходимое
        status_code: StatusCode,
        data: Option<Vec<u8>>
    ) -> Response<Body> {
        let mut header_map = HeaderMap::new();
        header_map.append(header::CONTENT_TYPE, HeaderValue::from_static(Self::HEADER_VALUE_CONTENT_TYPE));
        header_map.append(header::X_CONTENT_TYPE_OPTIONS, HeaderValue::from_static(Self::HEADER_VALUE_X_CONTENT_TYPE_OPTIONS));

        let mut parts = Response::new(()).into_parts().0;
        parts.status = status_code;
        parts.version = Version::HTTP_2;
        parts.headers = header_map;
        
        let body = match data {
            Some(data_) => Body::from(data_),
            None => Body::empty()
        };

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

    pub fn create_not_found(
    ) -> Response<Body> {
        return Self::create(StatusCode::NOT_FOUND, None);
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
}