use super::Inner;
use crate::{
    infrastructure_layer::{
        data::control_type::Response,
        functionality::service::{
            creator::Creator,
            formatter::action_round::RowData,
            logger::Logger,
        },
    },
    presentation_layer::functionality::{
        action::Action,
        service::processor::action_round::ActionRound,
    },
};
pub struct RouteNotFound;
impl Action<RouteNotFound> {
    pub fn run<'a>(inner: &'a mut Inner<'_>) -> Response {
        let response = Creator::<Response>::create_not_found();
        Logger::<ActionRound>::log(
            RowData {
                request_path: inner.parts.uri.path().to_string(),
                request_method: inner.parts.method.clone(),
                response_status_code: response.status().as_u16(),
            },
        );
        return response;
    }
}
