use super::{
    Formatter,
    Formatter_,
};
use crate::infrastructure_layer::data::{
    alternative_workflow::AlternativeWorkflow,
};
impl Formatter<AlternativeWorkflow> {
    pub fn format<'a>(error: &'a AlternativeWorkflow) -> String {
        return Formatter_::<AlternativeWorkflow>::format(error);
    }
}
