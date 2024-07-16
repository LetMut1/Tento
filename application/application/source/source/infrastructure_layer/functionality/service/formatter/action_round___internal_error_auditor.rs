use super::{
    context_report,
    Formatter,
};
use crate::infrastructure_layer::data::{
    alternative_workflow::InternalError,
    auditor::Auditor,
    control_type::ActionRound,
};
impl
    Formatter<(
        ActionRound,
        Auditor<InternalError>,
    )>
{
    pub fn format<'a>(request_uri: &'a str, request_method: &'a str, response_status_code: u16, internal_error_auditor: &'a Auditor<InternalError>) -> String {
        return format!(
            context_report!(),
            response_status_code,
            request_method,
            request_uri,
            Formatter::<Auditor<InternalError>>::format(internal_error_auditor).as_str(),
        );
    }
}
