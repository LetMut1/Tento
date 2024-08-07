use super::{
    context_report,
    Formatter,
    Formatter_,
};
use crate::infrastructure_layer::data::{
    control_type::ActionRound,
    server_workflow_error::{
        Expected, ExpectedInvalidArgument, Unexpected, UnexpectedInvalidArgument
    },
};
use http::method::Method;
use http::uri::PathAndQuery;
use aggregate_error::{
    Auditor,
    Backtrace,
    Logic,
    Runtime,
};
impl Formatter<ActionRound> {
    pub fn format<'a>(row_data: &'a RowData) -> String {
        return format!(
            base_report!(),
            row_data.response_status_code,
            row_data.request_method.as_str(),
            row_data.request_path.as_str(),
        );
    }
    pub fn format_unexpected_auditor<'a>(row_data: &'a RowData, unexpected_auditor: &'a Auditor<Unexpected>) -> String {
        let error_message = match unexpected_auditor.subject {
            Unexpected::Logic {
                ref logic,
            } => Formatter_::<Logic>::format(logic),
            Unexpected::Runtime {
                ref runtime,
            } => Formatter_::<Runtime>::format(runtime),
            Unexpected::InvalidArgument {
                ref unexpected_invalid_argument
            } => Formatter::<UnexpectedInvalidArgument>::format(unexpected_invalid_argument),
        };
        return format!(
            context_report!(),
            format!(
                context_report!(),
                Self::format(row_data).as_str(),
                error_message.as_str(),
            ).as_str(),
            Formatter_::<Backtrace>::format(&unexpected_auditor.backtrace),
        );
    }
    pub fn format_expected_auditor<'a>(row_data: &'a RowData, expected_auditor: &'a Auditor<Expected>) -> String {
        return format!(
            context_report!(),
            format!(
                context_report!(),
                Self::format(row_data).as_str(),
                Formatter::<ExpectedInvalidArgument>::format(&expected_auditor.subject.expected_invalid_argument).as_str(),
            ).as_str(),
            Formatter_::<Backtrace>::format(&expected_auditor.backtrace),
        );
    }
}
pub struct RowData {
    pub request_path: String,
    pub request_method: Method,
    pub response_status_code: u16,
}
macro_rules! base_report {
    () => {
        "\'{} {} {}\'"
    };
}
use base_report;
