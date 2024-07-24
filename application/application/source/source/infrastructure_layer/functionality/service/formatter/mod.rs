pub mod action_round;
pub mod unexpected_invalid_argument;
pub mod expected_invalid_argument;
use formatter::context_report;
use std::marker::PhantomData;
pub use formatter::Formatter as Formatter_;
pub struct Formatter<S> {
    _subject: PhantomData<S>,
}
