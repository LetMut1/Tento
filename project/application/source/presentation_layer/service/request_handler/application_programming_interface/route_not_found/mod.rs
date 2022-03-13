use crate::presentation_layer::service::response_creator::ResponseCreator;
use hyper::Body;
use hyper::Response;

pub struct RouteNotFound;

impl RouteNotFound {
    pub async fn not_found(
    ) -> Response<Body> {
        return ResponseCreator::create_not_found();
    }
}