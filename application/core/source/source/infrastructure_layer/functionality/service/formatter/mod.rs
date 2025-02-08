mod action_round;
mod aggregate_error;
mod backtrace;
mod indefinite_argument_context;
mod invalid_argument;
mod logic_context;
mod responsive;
mod runtime_context;
mod unresponsive;
pub use self::action_round::RowData;
use std::marker::PhantomData;
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
pub(crate) use {
    report_variant_1,
    report_variant_2,
};
