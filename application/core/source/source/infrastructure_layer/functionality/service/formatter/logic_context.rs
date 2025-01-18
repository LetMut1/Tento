use super::{
    report_variant_1,
    Formatter,
};
use crate::infrastructure_layer::data::aggregate_error::{
    Context,
    Logic,
};
use std::marker::PhantomData;
impl Formatter<Context<PhantomData<Logic>>> {
    pub fn format<'a>(invalid_argument_context: &'a Context<PhantomData<Logic>>) -> String {
        return format!(
            report_variant_1!(),
            "Logic",
            invalid_argument_context.error.as_ref(),
        );
    }
}
