use super::Formatter;
use crate::infrastructure_layer::data::auditor::Auditor;
use crate::infrastructure_layer::data::invalid_argument::InvalidArgument;
use super::context_report;
use crate::infrastructure_layer::data::control_type::ActionRound;

impl Formatter<(ActionRound, Auditor<InvalidArgument>)> {
    pub fn format<'a>(
        request_uri: &'a str,
        request_method: &'a str,
        response_status_code: u16,
        invalid_argument_auditor: &'a Auditor<InvalidArgument>,
    ) -> String {
        return format!(
            context_report!(),
            response_status_code,
            request_method,
            request_uri,
            Formatter::<Auditor<InvalidArgument>>::format(invalid_argument_auditor).as_str(),
        );
    }
}