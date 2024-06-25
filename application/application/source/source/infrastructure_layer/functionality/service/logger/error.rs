use super::Logger;
pub use crate::infrastructure_layer::data::{
    alternative_workflow::AlternativeWorkflow,
};
use crate::infrastructure_layer::functionality::service::formatter::Formatter;
impl Logger<AlternativeWorkflow> {
    pub fn log<'a>(error: &'a AlternativeWorkflow) -> () {
        tracing::error!(
            "{}",
            Formatter::<AlternativeWorkflow>::format(error).as_str(),
        );
        return ();
    }
}