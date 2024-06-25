use super::Logger;
use crate::infrastructure_layer::{
    data::{
        auditor::Auditor,
        control_type::{
            ActionRound,
            Response,
            TokioNonBlockingTask,
        },
        alternative_workflow::External,
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
        Auditor<External>,
    )>
{
    pub fn log<'a>(request_parts: &'a Parts, response: &'a Response, external_auditor: Auditor<External>) -> () {
        let request_uri = request_parts.uri.path().to_string();
        let request_method = request_parts.method.to_string();
        let response_status_code = response.status().as_u16();
        Spawner::<TokioNonBlockingTask>::spawn_into_background(
            async move {
                tracing::error!(
                    "{}",
                    Formatter::<(ActionRound, Auditor<External>)>::format(
                        request_uri.as_str(),
                        request_method.as_str(),
                        response_status_code,
                        &external_auditor,
                    )
                    .as_str()
                );
                return Ok(());
            },
        );
        return ();
    }
}
