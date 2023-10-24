use crate::infrastructure_layer::data::control_type::Response;
use crate::presentation_layer::functionality::action::Action;
use http::request::Parts;
use crate::application_layer::functionality::action_processor::ActionProcessor;
use crate::infrastructure_layer::data::control_type::ActionRound;
use crate::application_layer::functionality::service::reactor::Reactor;

pub use crate::application_layer::functionality::action_processor::route_not_found::RouteNotFound;

impl Action<RouteNotFound> {
    pub async fn run<'a>(
        parts: &'a Parts
    ) -> Response {
        let response = ActionProcessor::<RouteNotFound>::process().await;

        Reactor::<(ActionRound, Response)>::react(parts, &response);

        return response;
    }
}
