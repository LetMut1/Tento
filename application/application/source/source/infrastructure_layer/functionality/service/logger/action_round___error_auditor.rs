use crate::infrastructure_layer::functionality::service::formatter::Formatter;
use super::Logger;
use crate::infrastructure_layer::data::auditor::Auditor;
use crate::infrastructure_layer::data::error::Error;
use crate::infrastructure_layer::data::control_type::ActionRound;

impl Logger<(ActionRound, Auditor<Error>)> {
    pub fn log<'a>(
        request_uri: &'a str,
        request_method: &'a str,
        response_status_code: u16,
        error_auditor: &'a Auditor<Error>,
    ) -> () {
        tracing::error!(
            "{}",
            Formatter::<(ActionRound, Auditor<Error>)>::format(
                request_uri,
                request_method,
                response_status_code,
                error_auditor,
            ).as_str()
        );

        return ();
    }
}