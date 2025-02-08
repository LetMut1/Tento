use {
    super::{
        Format,
        Formatter,
    },
    crate::infrastructure_layer::data::{
        aggregate_error::{
            Context,
            Logic,
            Runtime,
        },
        server_workflow_error::Unresponsive,
    },
    std::marker::PhantomData,
};
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
