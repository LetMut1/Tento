use super::{
    Formatter,
    Formatter_,
};
use crate::infrastructure_layer::data::{
    alternative_workflow::InvalidArgument,
    auditor::Auditor,
};
impl Formatter<Auditor<InvalidArgument>> {
    pub fn format<'a>(invalid_argument_auditor: &'a Auditor<InvalidArgument>) -> String {
        return Formatter_::<Auditor<InvalidArgument>>::format(invalid_argument_auditor);
    }
}
