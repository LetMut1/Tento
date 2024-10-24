use super::{
    report_variant_1,
    Formatter,
};
use crate::infrastructure_layer::data::aggregate_error::{
    Context,
    IndefiniteArgument,
};
use std::marker::PhantomData;
impl Formatter<Context<PhantomData<IndefiniteArgument>>> {
    pub fn format<'a>(indefinite_argument_context: &'a Context<PhantomData<IndefiniteArgument>>) -> String {
        return format!(
            report_variant_1!(),
            "Indefinite argument",
            indefinite_argument_context.error.as_ref(),
        );
    }
}
