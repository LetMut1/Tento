pub mod action_log;
pub mod invalid_argument;
pub mod error_auditor;

use std::marker::PhantomData;

pub use formatter::Formatter as Formatter_;
pub use formatter::Format as Format_;

pub struct Formatter<S> {
    _subject: PhantomData<S>,
}