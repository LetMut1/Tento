pub mod action_round;
pub mod invalid_argument;
pub mod error_auditor;
pub mod backtrace;

use formatter::Formatter as Formatter_;
use std::marker::PhantomData;

pub struct Formatter<S> {
    _subject: PhantomData<S>,
}