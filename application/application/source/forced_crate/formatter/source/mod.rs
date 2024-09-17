mod aggregate_error;
mod backtrace;
mod indefinite_argument_context;
mod invalid_argument;
mod logic_context;
mod runtime_context;
use std::marker::PhantomData;
pub struct Formatter<S> {
    _subject: PhantomData<S>,
}
#[macro_export]
macro_rules! report_variant_1 {
    () => {
        "{} ({})"
    };
}
#[macro_export]
macro_rules! report_variant_2 {
    () => {
        "{}\n{}"
    };
}
