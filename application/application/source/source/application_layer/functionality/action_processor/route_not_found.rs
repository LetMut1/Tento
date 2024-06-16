use crate::{
    application_layer::functionality::action_processor::ActionProcessor,
    infrastructure_layer::{
        data::control_type::{
            Response,
            RouteNotFound,
        },
        functionality::service::creator::Creator,
    },
};
impl ActionProcessor<RouteNotFound> {
    pub fn process() -> Response {
        return Creator::<Response>::create_not_found();
    }
}
