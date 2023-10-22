use crate::infrastructure_layer::data::control_type::Response;
use crate::presentation_layer::functionality::action::Action;
use http::request::Parts;
use crate::application_layer::functionality::action_processor::ActionProcessor;

pub use crate::application_layer::functionality::action_processor::route_not_found::RouteNotFound;

impl Action<RouteNotFound> {
    pub async fn run<'a>(
        parts: &'a Parts
    ) -> Response {
        return ActionProcessor::<RouteNotFound>::process(parts).await;
    }
}
