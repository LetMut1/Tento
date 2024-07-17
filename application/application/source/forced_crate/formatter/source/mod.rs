mod aggregate_error;
mod backtrace;
mod invalid_argument;
mod logic;
mod runtime;
use std::marker::PhantomData;
pub struct Formatter<S> {
    _subject: PhantomData<S>,
}
#[macro_export]
macro_rules! context_report {
    () => {
        "{}:\n{}"
    };
}
