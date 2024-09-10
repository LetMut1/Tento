use super::{
    report_variant_1,
    Formatter,
};
use aggregate_error::{
    Context,
    Runtime,
};
use std::marker::PhantomData;
impl Formatter<Context<PhantomData<Runtime>>> {
    pub fn format<'a>(runtime_context: &'a Context<PhantomData<Runtime>>) -> String {
        return format!(
            report_variant_1!(),
            "Runtime",
            runtime_context.error.as_ref(),
        );
    }
}
