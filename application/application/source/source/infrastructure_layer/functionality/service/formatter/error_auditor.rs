use super::Formatter;
use super::Formatter_;
use crate::infrastructure_layer::data::auditor::Auditor;
use crate::infrastructure_layer::data::error::Error;
impl Formatter<Auditor<Error>> {
    pub fn format<'a>(error_auditor: &'a Auditor<Error>) -> String {
        return Formatter_::<Auditor<Error>>::format(error_auditor);
    }
}
