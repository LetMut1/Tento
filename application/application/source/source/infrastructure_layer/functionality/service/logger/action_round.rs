use super::Logger;
use crate::infrastructure_layer::{
    data::{control_type::{
        ActionRound,
        Response,
        TokioNonBlockingTask,
    }, server_workflow_error::{Expected, Unexpected},
    aggregate_error::Auditor,
    },
    functionality::service::{
        formatter::Formatter,
        spawner::Spawner,
    },
};
use http::request::Parts;
impl Logger<ActionRound> {
    pub fn log<'a>(request_parts: &'a Parts, response: &'a Response) -> () {
        let request_uri = request_parts.uri.path().to_string();
        let request_method = request_parts.method.to_string();
        let response_status_code = response.status().as_u16();
        Spawner::<TokioNonBlockingTask>::spawn_into_background(
            async move {
                tracing::info!(
                    "{}",
                    Formatter::<ActionRound>::format(
                        request_uri.as_str(),
                        request_method.as_str(),
                        response_status_code,
                    )
                    .as_str(),
                );
                return Ok(());
            },
        );
        return ();
    }
    pub fn log_unexpected_auditor<'a>(request_parts: &'a Parts, response: &'a Response, unexpected_auditor: Auditor<Unexpected>) -> () {
        let request_uri = request_parts.uri.path().to_string();
        let request_method = request_parts.method.to_string();
        let response_status_code = response.status().as_u16();
        Spawner::<TokioNonBlockingTask>::spawn_into_background(
            async move {
                tracing::error!(
                    "{}",
                    Formatter::<ActionRound>::format_unexpected_auditor(
                        request_uri.as_str(),
                        request_method.as_str(),
                        response_status_code,
                        &unexpected_auditor,
                    )
                    .as_str(),
                );
                return Ok(());
            },
        );
        return ();
    }
    pub fn log_expected_auditor<'a>(request_parts: &'a Parts, response: &'a Response, expected_auditor: Auditor<Expected>) -> () {
        let request_uri = request_parts.uri.path().to_string();
        let request_method = request_parts.method.to_string();
        let response_status_code = response.status().as_u16();
        Spawner::<TokioNonBlockingTask>::spawn_into_background(
            async move {
                tracing::info!(
                    "{}",
                    Formatter::<ActionRound>::format_expected_auditor(
                        request_uri.as_str(),
                        request_method.as_str(),
                        response_status_code,
                        &expected_auditor,
                    )
                    .as_str(),
                );
                return Ok(());
            },
        );
        return ();
    }
}
