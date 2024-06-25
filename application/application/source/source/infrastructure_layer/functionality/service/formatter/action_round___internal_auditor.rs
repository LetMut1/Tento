use super::{
    context_report,
    Formatter,
};
use crate::infrastructure_layer::data::{
    auditor::Auditor,
    control_type::ActionRound,
    alternative_workflow::Internal
};
impl
    Formatter<(
        ActionRound,
        Auditor<Internal>,
    )>
{
    pub fn format<'a>(request_uri: &'a str, request_method: &'a str, response_status_code: u16, internal_auditor: &'a Auditor<Internal>) -> String {
        return format!(
            context_report!(),
            response_status_code,
            request_method,
            request_uri,
            Formatter::<Auditor<Internal>>::format(internal_auditor).as_str(),
        );
    }
}
