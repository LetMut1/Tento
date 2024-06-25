use super::Formatter;
use auditor::{
    Auditor,
    Backtrace,
};
use alternative_workflow::{
    External,
};
impl Formatter<Auditor<External>> {
    pub fn format<'a>(external_auditor: &'a Auditor<External>) -> String {
        let message_part = match external_auditor.subject {
            External::InvalidArgument => "Invalid argument",
        };
        return format!(
            "{}.\n{}",
            message_part,
            Formatter::<Backtrace>::format(&external_auditor.backtrace).as_str(),
        );
    }
}
