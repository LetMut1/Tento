use super::Formatter;
use alternative_workflow::{
    AlternativeWorkflow,
    External,
    Internal,
};
use auditor::Auditor;
impl Formatter<AlternativeWorkflow> {
    pub fn format<'a>(alternative_workflow: &'a AlternativeWorkflow) -> String {
        return match *alternative_workflow {
            AlternativeWorkflow::Internal {
                ref internal_auditor,
            } => Formatter::<Auditor<Internal>>::format(internal_auditor),
            AlternativeWorkflow::External {
                ref external_auditor,
            } => Formatter::<Auditor<External>>::format(external_auditor),
        };
    }
}
