use aggregate_error::{Auditor, Backtrace, InvalidArgument, Logic, Runtime};
use super::{
    Formatter,
    context_report,
};
use crate::infrastructure_layer::data::{
    control_type::ActionRound,
    server_workflow_error::{Expected, Unexpected}
};
impl Formatter<ActionRound> {
    pub fn format<'a>(request_uri: &'a str, request_method: &'a str, response_status_code: u16) -> String {
        return format!(
            base_report!(),
            response_status_code,
            request_method,
            request_uri,
        );
    }
    pub fn format_unexpected_auditor<'a>(request_uri: &'a str, request_method: &'a str, response_status_code: u16, unexpected_auditor: &'a Auditor<Unexpected>) -> String {
        let error_message = match unexpected_auditor.subject {
            Unexpected::Logic {
                ref logic
            } => Formatter::<Logic>::format(logic),
            Unexpected::Runtime {
                ref runtime
            } => Formatter::<Runtime>::format(runtime),
        };

        return format!(
            context_report!(),
            format!(
                context_report!(),
                format!(
                    base_report!(),
                    response_status_code,
                    request_method,
                    request_uri,
                ).as_str(),
                error_message.as_str(),
            ).as_str(),
            Formatter::<Backtrace>::format(&unexpected_auditor.backtrace),
        );
    }
    pub fn format_expected_auditor<'a>(request_uri: &'a str, request_method: &'a str, response_status_code: u16, expected_auditor: &'a Auditor<Expected>) -> String {
        return format!(
            context_report!(),
            format!(
                context_report!(),
                format!(
                    base_report!(),
                    response_status_code,
                    request_method,
                    request_uri,
                ).as_str(),
                Formatter::<InvalidArgument>::format(&expected_auditor.subject.invalid_argument).as_str(),
            ).as_str(),
            Formatter::<Backtrace>::format(&expected_auditor.backtrace),
        );
    }
}
macro_rules! base_report {
    () => {
        "\'{} {} {}\'"
    };
}
use base_report;