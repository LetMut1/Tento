use super::Logger;
use crate::infrastructure_layer::{
    data::{
        auditor::Auditor,
        control_type::{
            ActionRound,
            Response,
            TokioNonBlockingTask,
        },
        error::Error,
    },
    functionality::service::{
        formatter::Formatter,
        spawner::Spawner,
    },
};
use http::request::Parts;
impl Logger<(ActionRound, Auditor<Error>)> {
    pub fn log<'a>(request_parts: &'a Parts, response: &'a Response, error_auditor: Auditor<Error>) -> () {
        let request_uri = request_parts.uri.path().to_string();
        let request_method = request_parts.method.to_string();
        let response_status_code = response.status().as_u16();
        let future = async move {
            tracing::error!(
                "{}",
                Formatter::<(ActionRound, Auditor<Error>)>::format(
                    request_uri.as_str(),
                    request_method.as_str(),
                    response_status_code,
                    &error_auditor,
                )
                .as_str()
            );
            return Ok(());
        };
        Spawner::<TokioNonBlockingTask>::spawn_into_background(future);
        return ();
    }
}
