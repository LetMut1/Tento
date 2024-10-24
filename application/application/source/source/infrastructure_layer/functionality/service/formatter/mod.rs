mod action_round;
mod responsive;
mod unresponsive;
mod aggregate_error;
mod backtrace;
mod indefinite_argument_context;
mod invalid_argument;
mod logic_context;
mod runtime_context;
use std::marker::PhantomData;
pub use self::action_round::RowData;
pub struct Formatter<S> {
    _subject: PhantomData<S>,
}
pub trait Format<S> {
    fn format<'a>(subject: &'a S) -> String;
}
macro_rules! report_variant_1 {
    () => {
        "{} ({})"
    };
}
macro_rules! report_variant_2 {
    () => {
        "{}\n{}"
    };
}
pub(crate) use report_variant_1;
pub(crate) use report_variant_2;
