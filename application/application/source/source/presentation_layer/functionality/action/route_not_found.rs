use crate::{
    infrastructure_layer::{
        data::control_type::{
            ActionRound,
            Response,
            RouteNotFound,
        },
        functionality::service::{creator::Creator, logger::Logger},
    },
    presentation_layer::functionality::action::Action,
};
use http::request::Parts;
impl Action<RouteNotFound> {
    pub fn run<'a>(parts: &'a Parts) -> Response {
        let response = Creator::<Response>::create_not_found();
        Logger::<ActionRound>::log(
            parts,
            &response,
        );
        return response;
    }
}
