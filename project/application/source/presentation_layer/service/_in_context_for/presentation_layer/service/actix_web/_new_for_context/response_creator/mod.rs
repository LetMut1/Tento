use actix_web::body::Body;
use actix_web::http::header;
use actix_web::HttpResponse;

pub struct ResponseCreator;

impl ResponseCreator {
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

    pub fn create_ok(
        body: Vec<u8>
    ) -> HttpResponse<Body> {
        return HttpResponse::Ok()
            .set_header(header::CONTENT_TYPE, "application/octet-stream")
            .set_header(header::X_CONTENT_TYPE_OPTIONS, "nosniff")
            .body(body);
    }
}