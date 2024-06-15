use crate::application_layer::functionality::action_processor::ActionProcessor;
pub use crate::infrastructure_layer::data::control_type::RouteNotFound;
use crate::infrastructure_layer::functionality::service::creator::response::Response;
use crate::infrastructure_layer::functionality::service::creator::Creator;
impl ActionProcessor<RouteNotFound> {
    pub fn process() -> Response {
        return Creator::<Response>::create_not_found();
    }
}
