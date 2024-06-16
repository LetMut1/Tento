use super::Logger;
use crate::infrastructure_layer::{
    data::{
        auditor::Auditor,
        control_type::{
            ActionRound,
            Response,
            TokioNonBlockingTask,
        },
        invalid_argument::InvalidArgument,
    },
    functionality::service::{
        formatter::Formatter,
        spawner::Spawner,
    },
};
use http::request::Parts;
impl Logger<(ActionRound, Auditor<InvalidArgument>)> {
    pub fn log<'a>(request_parts: &'a Parts, response: &'a Response, invalid_argument_auditor: Auditor<InvalidArgument>) -> () {
        let request_uri = request_parts.uri.path().to_string();
        let request_method = request_parts.method.to_string();
        let response_status_code = response.status().as_u16();
        let future = async move {
            tracing::error!(
                "{}",
                Formatter::<(ActionRound, Auditor<InvalidArgument>)>::format(
                    request_uri.as_str(),
                    request_method.as_str(),
                    response_status_code,
                    &invalid_argument_auditor,
                )
                .as_str()
            );
            return Ok(());
        };
        Spawner::<TokioNonBlockingTask>::spawn_into_background(future);
        return ();
    }
}
