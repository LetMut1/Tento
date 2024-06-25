use super::Formatter;
use auditor::{
    Auditor,
    Backtrace,
};
use alternative_workflow::{
    Internal,
};
impl Formatter<Auditor<Internal>> {
    pub fn format<'a>(internal_auditor: &'a Auditor<Internal>) -> String {
        let message_part = match internal_auditor.subject {
            Internal::Logic {
                message,
            } => format!(
                "Logic: {}.",
                message
            ),
            Internal::Runtime {
                ref runtime,
            } => format!(
                "Runtime: {}.",
                runtime.get()
            ),
        };
        return format!(
            "{}:\n{}",
            message_part.as_str(),
            Formatter::<Backtrace>::format(&internal_auditor.backtrace).as_str(),
        );
    }
}
