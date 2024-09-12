use super::{
    Format,
    Formatter,
};
use crate::infrastructure_layer::data::control_type::ActionRound;
use aggregate_error::{
    Auditor,
    Backtrace,
};
use formatter::{
    report_variant_2,
    Formatter as Formatter_,
};
use http_x::method::Method;
impl Formatter<ActionRound> {
    pub fn format<'a>(row_data: &'a RowData) -> String {
        return format!(
            "{} {} {}",
            row_data.response_status_code,
            row_data.request_method.as_str(),
            row_data.request_path.as_str(),
        );
    }
    pub fn format_with<'a, S>(row_data: &'a RowData, subject_auditor: &'a Auditor<S>) -> String
    where
        Formatter<S>: Format<S>,
    {
        return format!(
            report_variant_2!(),
            format!(
                report_variant_2!(),
                Self::format(row_data).as_str(),
                Formatter::<S>::format(&subject_auditor.subject).as_str(),
            ).as_str(),
            Formatter_::<Backtrace>::format(&subject_auditor.backtrace),
        );
    }
}
pub struct RowData {
    pub request_path: String,
    pub request_method: Method,
    pub response_status_code: u16,
}
