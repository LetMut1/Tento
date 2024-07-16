use super::Logger;
use crate::infrastructure_layer::{
    data::{
        alternative_workflow::InternalError,
        auditor::Auditor,
        control_type::{
            ActionRound,
            Response,
            TokioNonBlockingTask,
        },
    },
    functionality::service::{
        formatter::Formatter,
        spawner::Spawner,
    },
};
use http::request::Parts;
impl
    Logger<(
        ActionRound,
        Auditor<InternalError>,
    )>
{
    pub fn log<'a>(request_parts: &'a Parts, response: &'a Response, internal_error_auditor: Auditor<InternalError>) -> () {
        let request_uri = request_parts.uri.path().to_string();
        let request_method = request_parts.method.to_string();
        let response_status_code = response.status().as_u16();
        Spawner::<TokioNonBlockingTask>::spawn_into_background(
            async move {
                tracing::error!(
                    "{}",
                    Formatter::<(ActionRound, Auditor<InternalError>)>::format(
                        request_uri.as_str(),
                        request_method.as_str(),
                        response_status_code,
                        &internal_error_auditor,
                    )
                    .as_str()
                );
                return Ok(());
            },
        );
        return ();
    }
}
