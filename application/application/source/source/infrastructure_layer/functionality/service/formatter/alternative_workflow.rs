use super::{
    Formatter,
    Formatter_,
};
use crate::infrastructure_layer::data::alternative_workflow::AlternativeWorkflow;
impl Formatter<AlternativeWorkflow> {
    pub fn format<'a>(alternative_workflow: &'a AlternativeWorkflow) -> String {
        return Formatter_::<AlternativeWorkflow>::format(alternative_workflow);
    }
}
