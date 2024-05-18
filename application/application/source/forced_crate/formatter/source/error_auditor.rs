use error::Error;
use auditor::Auditor;
use auditor::Backtrace;
use super::Formatter;

impl Formatter<Auditor<Error>> {
    pub fn format<'a>(error_auditor: &'a Auditor<Error>) -> String {
        let error_message = match error_auditor.subject {
            Error::Logic {
                message,
            } => format!(
                "Logic: {}.",
                message
            ),
            Error::Runtime {
                ref runtime,
            } => format!(
                "Runtime: {}.",
                runtime.get(),
            ),
        };

        return format!(
            "{}:\n{}",
            error_message.as_str(),
            Formatter::<Backtrace>::format(&error_auditor.backtrace).as_str(),
        );
    }
}