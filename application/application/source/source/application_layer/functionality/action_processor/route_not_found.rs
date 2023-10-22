use crate::infrastructure_layer::functionality::service::creator::Creator;
use crate::infrastructure_layer::functionality::service::creator::response::Response;
use crate::application_layer::functionality::service::reactor::Reactor;
use http::request::Parts;
use crate::application_layer::functionality::action_processor::ActionProcessor;

pub use crate::infrastructure_layer::data::control_type::RouteNotFound;

impl ActionProcessor<RouteNotFound> {
    pub async fn process<'a>(
        parts: &'a Parts,
    ) -> Response {
        let response = Creator::<Response>::create_not_found();

        Reactor::<Response>::react(parts, &response);

        return response;
    }
}
