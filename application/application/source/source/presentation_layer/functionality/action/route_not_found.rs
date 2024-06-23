use crate::{
    application_layer::functionality::action_processor::ActionProcessor,
    infrastructure_layer::{
        data::control_type::{
            ActionRound,
            Response,
            RouteNotFound,
        },
        functionality::service::logger::Logger,
    },
    presentation_layer::functionality::action::Action,
};
use http::request::Parts;
impl Action<RouteNotFound> {
    pub fn run<'a>(parts: &'a Parts) -> Response {
        let response = ActionProcessor::<RouteNotFound>::process();
        Logger::<(
            ActionRound,
            Response,
        )>::log(
            parts,
            &response,
        );
        return response;
    }
}
