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
    pub fn format<'a>(error: &'a AlternativeWorkflow) -> String {
        return match *error {
            AlternativeWorkflow::Internal { ref internal_auditor } => Formatter::<Auditor<Internal>>::format(internal_auditor),
            AlternativeWorkflow::External { ref external_auditor } => Formatter::<Auditor<External>>::format(external_auditor),
        };
    }
}
