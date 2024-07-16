use super::Formatter;
use alternative_workflow::InvalidArgument;
use auditor::{
    Auditor,
    Backtrace,
};
impl Formatter<Auditor<InvalidArgument>> {
    pub fn format<'a>(invalid_argument_auditor: &'a Auditor<InvalidArgument>) -> String {
        let message_part = match invalid_argument_auditor.subject {
            InvalidArgument::FromOutside
            | InvalidArgument::FromClientCode => "Invalid argument",
        };
        return format!(
            "{}.\n{}",
            message_part,
            Formatter::<Backtrace>::format(&invalid_argument_auditor.backtrace).as_str(),
        );
    }
}
