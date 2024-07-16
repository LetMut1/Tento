use super::{
    Formatter,
    Formatter_,
};
use crate::infrastructure_layer::data::{
    alternative_workflow::InternalError,
    auditor::Auditor,
};
impl Formatter<Auditor<InternalError>> {
    pub fn format<'a>(internal_error_auditor: &'a Auditor<InternalError>) -> String {
        return Formatter_::<Auditor<InternalError>>::format(internal_error_auditor);
    }
}
