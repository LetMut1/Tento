use actix_http::body::BoxBody;
use actix_web::http::header;
use actix_web::http::StatusCode;
use actix_web::HttpResponse;

pub struct ResponseCreator;

impl ResponseCreator {
    fn create(
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

    pub fn create_bad_request(
    ) -> HttpResponse<BoxBody> {
        return Self::create(StatusCode::BAD_REQUEST, None);
    }

    pub fn create_unauthorized(
    ) -> HttpResponse<BoxBody> {
        return Self::create(StatusCode::UNAUTHORIZED, None);
    }

    pub fn create_internal_server_error(
    ) -> HttpResponse<BoxBody> {
        return Self::create(StatusCode::INTERNAL_SERVER_ERROR, None);
    }

    pub fn create_ok(
        data: Vec<u8>
    ) -> HttpResponse<BoxBody> {
        return Self::create(StatusCode::OK, Some(data));
    }

    #[cfg(feature="facilitate_non_automatic_functional_testing")]
    pub fn create_(
        status_code: StatusCode,
        data: Option<Vec<u8>>
    ) -> HttpResponse<BoxBody> {
        return Self::create(status_code, data);
    }
}