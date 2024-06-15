use crate::infrastructure_layer::data::auditor::Auditor;
use crate::infrastructure_layer::data::control_type::{ActionRound, Response, TokioNonBlockingTask};
use crate::infrastructure_layer::data::error::Error;
use crate::infrastructure_layer::functionality::service::formatter::Formatter;
use crate::infrastructure_layer::functionality::service::spawner::Spawner;
use http::request::Parts;
use super::Logger;

impl Logger<(ActionRound, Auditor<Error>)> {
    pub fn log<'a>(
        request_parts: &'a Parts,
        response: &'a Response,
        error_auditor: Auditor<Error>
    ) -> () {
        let request_uri = request_parts.uri.path().to_string();

        let request_method = request_parts.method.to_string();

        let response_status_code = response.status().as_u16();

        Spawner::<TokioNonBlockingTask>::spawn_into_background(
            async move {
                tracing::error!(
                    "{}",
                    Formatter::<(ActionRound, Auditor<Error>)>::format(
                        request_uri.as_str(),
                        request_method.as_str(),
                        response_status_code,
                        &error_auditor,
                    ).as_str()
                );

                return Ok(());
            }
        );

        return ();
    }
}