use super::creator::Creator;
use crate::domain_layer::data::entity::action_round_register::ActionRoundRegister_Context;
use crate::infrastructure_layer::data::error_auditor::ErrorAuditor_;
use crate::infrastructure_layer::data::invalid_argument_result::InvalidArgument;
use crate::infrastructure_layer::functionality::service::formatter::Format;
use crate::infrastructure_layer::functionality::service::formatter::Formatter;

pub trait ContextFrom<T> {
    fn create<'a>(from: &'a T) -> ActionRoundRegister_Context;
}

impl ContextFrom<InvalidArgument> for Creator<ActionRoundRegister_Context> {
    fn create<'a>(from: &'a InvalidArgument) -> ActionRoundRegister_Context {
        return ActionRoundRegister_Context(Formatter::prepare(from));
    }
}

impl ContextFrom<ErrorAuditor_> for Creator<ActionRoundRegister_Context> {
    fn create<'a>(from: &'a ErrorAuditor_) -> ActionRoundRegister_Context {
        return ActionRoundRegister_Context(Formatter::prepare(from));
    }
}
