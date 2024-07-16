use super::Formatter;
use alternative_workflow::{
    AlternativeWorkflow,
    External,
    InternalError,
};
use auditor::Auditor;
impl Formatter<AlternativeWorkflow> {
    pub fn format<'a>(alternative_workflow: &'a AlternativeWorkflow) -> String {
        return match *alternative_workflow {
            AlternativeWorkflow::InternalError {
                ref internal_error_auditor,
            } => Formatter::<Auditor<InternalError>>::format(internal_error_auditor),
            AlternativeWorkflow::External {
                ref external_auditor,
            } => Formatter::<Auditor<External>>::format(external_auditor),
        };
    }
}
