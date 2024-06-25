use super::{
    Formatter,
    Formatter_,
};
use crate::infrastructure_layer::data::{
    alternative_workflow::Internal,
    auditor::Auditor,
};
impl Formatter<Auditor<Internal>> {
    pub fn format<'a>(internal_auditor: &'a Auditor<Internal>) -> String {
        return Formatter_::<Auditor<Internal>>::format(internal_auditor);
    }
}
