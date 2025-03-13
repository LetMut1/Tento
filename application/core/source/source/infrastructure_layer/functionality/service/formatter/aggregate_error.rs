use {
    super::{
        Formatter,
        report_variant_2,
    },
    crate::infrastructure_layer::data::aggregate_error::{
        AggregateError,
        AggregateError_,
        Backtrace,
        Context,
        IndefiniteArgument,
        InvalidArgument,
        Logic,
        Runtime,
    },
    std::marker::PhantomData,
};
impl Formatter<AggregateError> {
    pub fn format<'a>(aggregate_error: &'a AggregateError) -> String {
        let message_part = match aggregate_error.0.subject {
            AggregateError_::IndefiniteArgument {
                ref indefinite_argument_context,
            } => Formatter::<Context<PhantomData<IndefiniteArgument>>>::format(indefinite_argument_context),
            AggregateError_::InvalidArgument {
                invalid_argument: _,
            } => Formatter::<PhantomData<InvalidArgument>>::format(),
            AggregateError_::Logic {
                ref logic_context,
            } => Formatter::<Context<PhantomData<Logic>>>::format(logic_context),
            AggregateError_::Runtime {
                ref runtime_context,
            } => Formatter::<Context<PhantomData<Runtime>>>::format(runtime_context),
        };
        return format!(
            report_variant_2!(),
            message_part.as_str(),
            Formatter::<Backtrace>::format(&aggregate_error.0.backtrace).as_str(),
        );
    }
}
