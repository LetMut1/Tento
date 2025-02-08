use {
    crate::{
        infrastructure_layer::{
            data::control_type::Response,
            functionality::service::{
                creator::Creator,
                formatter::RowData,
                logger::Logger,
            },
        },
        presentation_layer::functionality::{
            action::Action,
            service::processor::action_round::ActionRound,
        },
    },
    super::Inner,
};
pub struct RouteNotFound;
impl Action<RouteNotFound> {
    pub fn run<'a>(inner: &'a Inner<'_>) -> Response {
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
