use crate::domain_layer::data::entity::action_round_register::ActionRoundRegister_Context;
use crate::infrastructure_layer::data::argument_result::InvalidArgument;
use crate::infrastructure_layer::data::error_auditor::ErrorAuditor;
use crate::infrastructure_layer::functionality::service::formatter::Format;
use crate::infrastructure_layer::functionality::service::formatter::Formatter;
use super::creator::Creator;

pub trait CreateContext<T> {
    fn create<'a>(from: &'a T) -> String;
}

impl CreateContext<InvalidArgument> for Creator<ActionRoundRegister_Context> {
    fn create<'a>(from: &'a InvalidArgument) -> String {
        return Formatter::prepare(from);
    }
}

impl CreateContext<ErrorAuditor> for Creator<ActionRoundRegister_Context> {
    fn create<'a>(from: &'a ErrorAuditor) -> String {
        return Formatter::prepare(from);
    }
}