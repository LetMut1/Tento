use crate::presentation_layer::service::action_response_creator::ActionResponseCreator;
use hyper::Body;
use hyper::Response;

pub struct RouteNotFound;

impl RouteNotFound {
    pub async fn not_found(
    ) -> Response<Body> {
        return ActionResponseCreator::create_not_found();
    }
}