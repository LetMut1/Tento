use actix_web::dev::Body;
use actix_web::http::header;
use actix_web::HttpResponse;
use super::json_response_body_wrapper::JsonResponseBodyWrapper;
use super::response_body_wrapper_trait::ResponseBodyWrapperTrait;
use super::response_creator_trait::ResponseCreatorTrait_;
use super::response_creator_trait::ResponseCreatorTrait;

pub struct JsonResponseCreator;

impl ResponseCreatorTrait for JsonResponseCreator {}

impl ResponseCreatorTrait_ for JsonResponseCreator {
    type ResponseBodyWrapper = JsonResponseBodyWrapper;

    fn create_ok(
        body: <Self::ResponseBodyWrapper as ResponseBodyWrapperTrait>::WrappedBodyType
    ) -> HttpResponse<Body> {
        return HttpResponse::Ok()
            .set_header(header::CONTENT_TYPE, "application/json")
            .set_header(header::X_CONTENT_TYPE_OPTIONS, "nosniff")
            .body(body);
    }
}