use actix_web::dev::Body;
use actix_web::http::header;
use actix_web::HttpResponse;
use super::byte_response_data_wrapper::ByteResponseDataWrapper;
use super::response_data_wrapper_trait::ResponseDataWrapperTrait;
use super::response_creator_trait::ResponseCreatorTrait_;
use super::response_creator_trait::ResponseCreatorTrait;

pub struct ByteResponseCreator;

impl ResponseCreatorTrait for ByteResponseCreator {}

impl ResponseCreatorTrait_ for ByteResponseCreator {
    type ResponseDataWrapper = ByteResponseDataWrapper;

    fn create_ok(
        body: <Self::ResponseDataWrapper as ResponseDataWrapperTrait>::WrappedDataType
    ) -> HttpResponse<Body> {
        return HttpResponse::Ok()
            .set_header(header::CONTENT_TYPE, "application/octet-stream")
            .set_header(header::X_CONTENT_TYPE_OPTIONS, "nosniff")
            .body(body);
    }
}