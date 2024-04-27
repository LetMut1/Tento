use crate::infrastructure_layer::data::auditor::Auditor;
use crate::infrastructure_layer::data::error::Error;
use super::Formatter_;
use super::Formatter;

impl Formatter<Auditor<Error>> {
    pub fn format<'a>(error_auditor: &'a Auditor<Error>) -> String {
        return Formatter_::<Auditor<Error>>::format(error_auditor);
    }
}