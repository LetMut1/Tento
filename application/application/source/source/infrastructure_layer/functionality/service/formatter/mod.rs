pub mod action_round;
pub mod error_auditor;
pub mod backtrace;
pub mod invalid_argument_auditor;

use formatter::Formatter as Formatter_;
use std::marker::PhantomData;

pub struct Formatter<S> {
    _subject: PhantomData<S>,
}