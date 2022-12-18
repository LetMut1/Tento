use crate::presentation_layer::functionality::service::action_response_creator::ActionResponseCreator;
use extern_crate::hyper::Body;
use extern_crate::hyper::Response;

pub struct RouteNotFound;

impl RouteNotFound {
    pub async fn not_found(
    ) -> Response<Body> {
        return ActionResponseCreator::create_not_found();
    }
}