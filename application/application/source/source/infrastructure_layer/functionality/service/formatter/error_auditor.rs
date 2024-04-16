use crate::infrastructure_layer::functionality::service::formatter::Format_;
use crate::infrastructure_layer::functionality::service::formatter::Formatter_;
use super::Formatter;
use crate::infrastructure_layer::data::error_auditor::Error;

pub use crate::infrastructure_layer::data::error_auditor::Auditor;

impl Formatter<Auditor<Error>> {
    pub fn format<'a>(error_auditor: &'a Auditor<Error>) -> String {
        todo!();
        // return Formatter_::prepare(error_auditor);
    }
}