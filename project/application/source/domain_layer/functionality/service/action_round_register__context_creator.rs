use crate::infrastructure_layer::data::error_auditor::ErrorAuditor;
use crate::infrastructure_layer::data::argument_result::InvalidArgument;
use crate::infrastructure_layer::functionality::service::displayer::Display;
use crate::infrastructure_layer::functionality::service::displayer::Displayer;

pub struct ActionRoundRegister_ContextCreator;

pub trait CreateContext<T> {
    fn create<'a>(from: &'a T) -> String;
}

impl CreateContext<InvalidArgument> for ActionRoundRegister_ContextCreator {
    fn create<'a>(from: &'a InvalidArgument) -> String {
        return Displayer::display(from);
    }
}

impl CreateContext<ErrorAuditor> for ActionRoundRegister_ContextCreator {
    fn create<'a>(from: &'a ErrorAuditor) -> String {
        return Displayer::display(from);
    }
}