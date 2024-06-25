use super::Logger;
use crate::infrastructure_layer::{
    data::{
        alternative_workflow::Internal,
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
        Auditor<Internal>,
    )>
{
    pub fn log<'a>(request_parts: &'a Parts, response: &'a Response, internal_auditor: Auditor<Internal>) -> () {
        let request_uri = request_parts.uri.path().to_string();
        let request_method = request_parts.method.to_string();
        let response_status_code = response.status().as_u16();
        Spawner::<TokioNonBlockingTask>::spawn_into_background(
            async move {
                tracing::error!(
                    "{}",
                    Formatter::<(ActionRound, Auditor<Internal>)>::format(
                        request_uri.as_str(),
                        request_method.as_str(),
                        response_status_code,
                        &internal_auditor,
                    )
                    .as_str()
                );
                return Ok(());
            },
        );
        return ();
    }
}
