use crate::infrastructure_layer::functionality::service::creator::response::Response;
use http::request::Parts;
use crate::infrastructure_layer::functionality::service::logger::Logger;
use super::Reactor;
use crate::infrastructure_layer::data::control_type::TokioNonBlockingTask;
use crate::infrastructure_layer::functionality::service::spawner::Spawner;

pub use crate::infrastructure_layer::data::control_type::ActionRound;
pub use crate::infrastructure_layer::data::invalid_argument_result::InvalidArgument;

impl Reactor<(ActionRound, InvalidArgument)> {
    pub fn react<'a>(
        request_parts: &'a Parts,
        response: &'a Response,
        invalid_argument: InvalidArgument
    ) -> () {
        let request_uri = request_parts.uri.path().to_string();

        let request_method = request_parts.method.to_string();

        let response_status_code = response.status().as_u16();

        Spawner::<TokioNonBlockingTask>::spawn_into_background(
            async move {
                Logger::<(ActionRound, InvalidArgument)>::log(
                    request_uri.as_str(),
                    request_method.as_str(),
                    response_status_code,
                    &invalid_argument,
                );

                return Ok(());
            }
        );

        return ();
    }
}
