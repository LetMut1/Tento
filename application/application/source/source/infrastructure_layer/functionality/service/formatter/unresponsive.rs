use super::{
    Format,
    Formatter,
};
use crate::infrastructure_layer::data::server_workflow_error::Unresponsive;
use aggregate_error::{
    Context,
    Logic,
    Runtime,
};
use formatter::Formatter as Formatter_;
use std::marker::PhantomData;
impl Format<Unresponsive> for Formatter<Unresponsive> {
    fn format<'a>(subject: &'a Unresponsive) -> String {
        return match *subject {
            Unresponsive::Logic {
                ref logic_context,
            } => Formatter_::<Context<PhantomData<Logic>>>::format(logic_context),
            Unresponsive::Runtime {
                ref runtime_context,
            } => Formatter_::<Context<PhantomData<Runtime>>>::format(runtime_context),
        };
    }
}
