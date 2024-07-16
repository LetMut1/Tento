use super::Formatter;
use aggregate_error::InternalError;
use auditor::{
    Auditor,
    Backtrace,
};
impl Formatter<Auditor<InternalError>> {
    pub fn format<'a>(internal_error_auditor: &'a Auditor<InternalError>) -> String {
        let message_part = match internal_error_auditor.subject {
            InternalError::Logic {
                message,
            } => {
                format!(
                    "Logic: {}.",
                    message
                )
            }
            InternalError::Runtime {
                ref runtime,
            } => {
                format!(
                    "Runtime: {}.",
                    runtime.get()
                )
            }
        };
        return format!(
            "{}:\n{}",
            message_part.as_str(),
            Formatter::<Backtrace>::format(&internal_error_auditor.backtrace).as_str(),
        );
    }
}
