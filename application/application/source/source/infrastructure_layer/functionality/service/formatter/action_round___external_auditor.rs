use super::{
    context_report,
    Formatter,
};
use crate::infrastructure_layer::data::{
    auditor::Auditor,
    control_type::ActionRound,
    alternative_workflow::External
};
impl
    Formatter<(
        ActionRound,
        Auditor<External>,
    )>
{
    pub fn format<'a>(request_uri: &'a str, request_method: &'a str, response_status_code: u16, external_auditor: &'a Auditor<External>) -> String {
        return format!(
            context_report!(),
            response_status_code,
            request_method,
            request_uri,
            Formatter::<Auditor<External>>::format(external_auditor).as_str(),
        );
    }
}
