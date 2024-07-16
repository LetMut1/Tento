use super::Formatter;
use aggregate_error::{
    AlternativeWorkflow,
    InvalidArgument,
    InternalError,
};
use auditor::Auditor;
impl Formatter<AlternativeWorkflow> {
    pub fn format<'a>(alternative_workflow: &'a AlternativeWorkflow) -> String {
        return match *alternative_workflow {
            AlternativeWorkflow::InternalError {
                ref internal_error_auditor,
            } => Formatter::<Auditor<InternalError>>::format(internal_error_auditor),
            AlternativeWorkflow::InvalidArgument {
                ref invalid_argument_auditor,
            } => Formatter::<Auditor<InvalidArgument>>::format(invalid_argument_auditor),
        };
    }
}
