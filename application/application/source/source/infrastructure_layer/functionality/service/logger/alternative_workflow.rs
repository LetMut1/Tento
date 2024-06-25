use super::Logger;
pub use crate::infrastructure_layer::data::{
    alternative_workflow::AlternativeWorkflow,
};
use crate::infrastructure_layer::functionality::service::formatter::Formatter;
impl Logger<AlternativeWorkflow> {
    pub fn log<'a>(alternative_workflow: &'a AlternativeWorkflow) -> () {
        tracing::error!(
            "{}",
            Formatter::<AlternativeWorkflow>::format(alternative_workflow).as_str(),
        );
        return ();
    }
}