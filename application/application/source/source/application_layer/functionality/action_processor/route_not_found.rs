pub use crate::infrastructure_layer::data::control_type::RouteNotFound;
use crate::{
    application_layer::functionality::action_processor::ActionProcessor,
    infrastructure_layer::functionality::service::creator::{
        response::Response,
        Creator,
    },
};
impl ActionProcessor<RouteNotFound> {
    pub fn process() -> Response {
        return Creator::<Response>::create_not_found();
    }
}
