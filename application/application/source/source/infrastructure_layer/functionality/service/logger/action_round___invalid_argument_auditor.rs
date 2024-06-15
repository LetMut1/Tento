use super::Logger;
use crate::infrastructure_layer::data::auditor::Auditor;
use crate::infrastructure_layer::data::control_type::{ActionRound, Response, TokioNonBlockingTask};
use crate::infrastructure_layer::data::invalid_argument::InvalidArgument;
use crate::infrastructure_layer::functionality::service::formatter::Formatter;
use crate::infrastructure_layer::functionality::service::spawner::Spawner;
use http::request::Parts;
impl Logger<(ActionRound, Auditor<InvalidArgument>)> {
    pub fn log<'a>(
        request_parts: &'a Parts,
        response: &'a Response,
        invalid_argument_auditor: Auditor<InvalidArgument>,
    ) -> () {
        let request_uri = request_parts.uri.path().to_string();
        let request_method = request_parts.method.to_string();
        let response_status_code = response.status().as_u16();
        Spawner::<TokioNonBlockingTask>::spawn_into_background(async move {
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
        });
        return ();
    }
}
