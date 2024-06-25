use super::Formatter;
use auditor::{
    Auditor,
};
use alternative_workflow::{
    AlternativeWorkflow,
    Internal,
    External,
};
impl Formatter<AlternativeWorkflow> {
    pub fn format<'a>(alternative_workflow: &'a AlternativeWorkflow) -> String {
        return match *alternative_workflow {
            AlternativeWorkflow::Internal { ref internal_auditor } => Formatter::<Auditor<Internal>>::format(internal_auditor),
            AlternativeWorkflow::External { ref external_auditor } => Formatter::<Auditor<External>>::format(external_auditor),
        };
    }
}
