pub mod action_round___error_auditor;
pub mod action_round___invalid_argument_auditor;
pub mod action_round___response;
pub mod backtrace;
pub mod error_auditor;
pub mod invalid_argument_auditor;

use formatter::Formatter as Formatter_;
use std::marker::PhantomData;

pub struct Formatter<S> {
    _subject: PhantomData<S>,
}

macro_rules! context_report {
    () => {
        "\'{} {} {}\':\n{}"
    };
}

use context_report;
