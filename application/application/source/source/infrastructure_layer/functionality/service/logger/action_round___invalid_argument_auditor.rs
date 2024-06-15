use crate::infrastructure_layer::functionality::service::formatter::Formatter;
use super::Logger;
use crate::infrastructure_layer::data::invalid_argument::InvalidArgument;
use crate::infrastructure_layer::data::auditor::Auditor;
use crate::infrastructure_layer::data::control_type::ActionRound;

impl Logger<(ActionRound, Auditor<InvalidArgument>)> {
    pub fn log<'a>(
        request_uri: &'a str,
        request_method: &'a str,
        response_status_code: u16,
        invalid_argument_auditor: &'a Auditor<InvalidArgument>,
    ) -> () {
        tracing::error!(
            "{}",
            Formatter::<(ActionRound, Auditor<InvalidArgument>)>::format(
                request_uri,
                request_method,
                response_status_code,
                invalid_argument_auditor,
            ).as_str()
        );

        return ();
    }
}