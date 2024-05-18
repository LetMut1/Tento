use crate::infrastructure_layer::data::auditor::Auditor;
use crate::infrastructure_layer::data::invalid_argument::InvalidArgument;
use crate::infrastructure_layer::data::auditor::Backtrace;
use super::Formatter_;
use super::Formatter;

impl Formatter<Auditor<InvalidArgument>> {
    pub fn format<'a>(invalid_argument_auditor: &'a Auditor<InvalidArgument>) -> String {
        return format!(
            "Invalid argument.\n{}",
            Formatter_::<Backtrace>::format(&invalid_argument_auditor.backtrace).as_str(),
        );
    }
}