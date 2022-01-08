use actix_web::dev::Body;
use actix_web::http::header;
use actix_web::HttpResponse;
use super::byte_response_body_wrapper::ByteResponseBodyWrapper;
use super::response_body_wrapper_trait::ResponseBodyWrapperTrait;
use super::response_creator_trait::ResponseCreatorTrait_;
use super::response_creator_trait::ResponseCreatorTrait;

pub struct ByteResponseCreator;

impl ResponseCreatorTrait for ByteResponseCreator {}

impl ResponseCreatorTrait_ for ByteResponseCreator {
    type ResponseBodyWrapper = ByteResponseBodyWrapper;

    fn create_ok(
        body: <Self::ResponseBodyWrapper as ResponseBodyWrapperTrait>::WrappedBodyType
    ) -> HttpResponse<Body> {
        return HttpResponse::Ok()
            .set_header(header::CONTENT_TYPE, "application/octet-stream")
            .set_header(header::X_CONTENT_TYPE_OPTIONS, "nosniff")
            .body(body);
    }
}