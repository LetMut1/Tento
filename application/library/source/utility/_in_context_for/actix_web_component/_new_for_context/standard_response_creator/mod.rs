use actix_web::dev::Body;
use actix_web::http::header;
use actix_web::HttpResponse;

pub struct StandardResponseCreator;

impl StandardResponseCreator {
    pub fn create_ok(body: String) -> HttpResponse<Body> {
        return HttpResponse::Ok()
        .set_header(header::CONTENT_TYPE, "application/json")
        .body(body);
    }

    pub fn create_unauthorized() -> HttpResponse<Body> {
        return HttpResponse::Unauthorized().finish();
    }

    pub fn create_bad_request() -> HttpResponse<Body> {
        return HttpResponse::BadRequest().finish();
    }

    pub fn create_internal_server_error() -> HttpResponse<Body> {
        return HttpResponse::InternalServerError().finish();
    }
}