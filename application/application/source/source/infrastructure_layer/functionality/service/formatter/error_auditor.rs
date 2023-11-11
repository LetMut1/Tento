use crate::infrastructure_layer::functionality::service::formatter::Format_;
use crate::infrastructure_layer::functionality::service::formatter::Formatter_;
use super::Formatter;

pub use crate::infrastructure_layer::data::error_auditor::ErrorAuditor;

impl Formatter<ErrorAuditor> {
    pub fn format<'a>(error_auditor: &'a ErrorAuditor) -> String {
        return Formatter_::prepare(error_auditor);
    }
}