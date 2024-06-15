use super::Formatter;
use crate::infrastructure_layer::data::auditor::Auditor;
use crate::infrastructure_layer::data::error::Error;
use super::context_report;
use crate::infrastructure_layer::data::control_type::ActionRound;

impl Formatter<(ActionRound, Auditor<Error>)> {
    pub fn format<'a>(
        request_uri: &'a str,
        request_method: &'a str,
        response_status_code: u16,
        error_auditor: &'a Auditor<Error>
    ) -> String {
        return format!(
            context_report!(),
            response_status_code,
            request_method,
            request_uri,
            Formatter::<Auditor<Error>>::format(error_auditor).as_str(),
        );
    }
}