use crate::application_layer::functionality::action_processor::ActionProcessor;
use crate::infrastructure_layer::data::control_type::ActionRound;
use crate::infrastructure_layer::data::control_type::Response;
use crate::infrastructure_layer::functionality::service::logger::Logger;
use crate::presentation_layer::functionality::action::Action;
use http::request::Parts;

pub use crate::application_layer::functionality::action_processor::route_not_found::RouteNotFound;

impl Action<RouteNotFound> {
    pub fn run<'a>(parts: &'a Parts) -> Response {
        let response = ActionProcessor::<RouteNotFound>::process();

        Logger::<(ActionRound, Response)>::log(parts, &response);

        return response;
    }
}
