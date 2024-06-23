use super::{
    context_report,
    Formatter,
};
use crate::infrastructure_layer::data::{
    auditor::Auditor,
    control_type::ActionRound,
    error::Error,
};
impl
    Formatter<(
        ActionRound,
        Auditor<Error>,
    )>
{
    pub fn format<'a>(request_uri: &'a str, request_method: &'a str, response_status_code: u16, error_auditor: &'a Auditor<Error>) -> String {
        return format!(
            context_report!(),
            response_status_code,
            request_method,
            request_uri,
            Formatter::<Auditor<Error>>::format(error_auditor).as_str(),
        );
    }
}
