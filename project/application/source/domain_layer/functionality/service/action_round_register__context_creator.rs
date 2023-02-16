use crate::infrastructure_layer::data::error_auditor::ErrorAuditor;
use crate::infrastructure_layer::data::invalid_argument::InvalidArgument;
use crate::infrastructure_layer::functionality::service::displayer::Display;
use crate::infrastructure_layer::functionality::service::displayer::Displayer;

pub trait ActionRoundRegister_CreateContext<T> {
    type Displayer_: Display<T>;

    fn create<'a>(from: &'a T) -> String {
        return <Self::Displayer_ as Display<T>>::display(from);
    }
}

pub struct ActionRoundRegister_ContextCreator;

impl ActionRoundRegister_CreateContext<InvalidArgument> for ActionRoundRegister_ContextCreator {
    type Displayer_ = Displayer;
}

impl ActionRoundRegister_CreateContext<ErrorAuditor> for ActionRoundRegister_ContextCreator {
    type Displayer_ = Displayer;
}