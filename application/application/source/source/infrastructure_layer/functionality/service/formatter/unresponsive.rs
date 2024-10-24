use super::{
    Format,
    Formatter,
};
use crate::infrastructure_layer::data::server_workflow_error::Unresponsive;
use crate::infrastructure_layer::data::aggregate_error::{
    Context,
    Logic,
    Runtime,
};
use std::marker::PhantomData;
impl Format<Unresponsive> for Formatter<Unresponsive> {
    fn format<'a>(subject: &'a Unresponsive) -> String {
        return match *subject {
            Unresponsive::Logic {
                ref logic_context,
            } => Formatter::<Context<PhantomData<Logic>>>::format(logic_context),
            Unresponsive::Runtime {
                ref runtime_context,
            } => Formatter::<Context<PhantomData<Runtime>>>::format(runtime_context),
        };
    }
}
