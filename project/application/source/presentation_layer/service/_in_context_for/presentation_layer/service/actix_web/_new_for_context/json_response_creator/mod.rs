use actix_web::dev::Body;
use actix_web::http::header;
use actix_web::HttpResponse;
use super::json_response_data_wrapper::JsonResponseDataWrapper;
use super::response_data_wrapper_trait::ResponseDataWrapperTrait;
use super::response_creator_trait::ResponseCreatorTrait_;
use super::response_creator_trait::ResponseCreatorTrait;

pub struct JsonResponseCreator;

impl ResponseCreatorTrait for JsonResponseCreator {}

impl ResponseCreatorTrait_ for JsonResponseCreator {
    type ResponseDataWrapper = JsonResponseDataWrapper;

    fn create_ok(
        body: <Self::ResponseDataWrapper as ResponseDataWrapperTrait>::WrappedDataType
    ) -> HttpResponse<Body> {
        return HttpResponse::Ok()
            .set_header(header::CONTENT_TYPE, "application/json")
            .set_header(header::X_CONTENT_TYPE_OPTIONS, "nosniff")
            .body(body);
    }
}