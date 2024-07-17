pub mod action_round;
pub mod aggregate_error;
pub mod backtrace;
pub mod invalid_argument;
pub mod logic;
pub mod runtime;
use formatter::{
    context_report,
    Formatter as Formatter_,
};
use std::marker::PhantomData;
pub struct Formatter<S> {
    _subject: PhantomData<S>,
}
