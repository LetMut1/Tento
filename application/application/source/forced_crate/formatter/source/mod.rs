pub mod aggregate_error;
mod backtrace;
mod invalid_argument_auditor;
mod internal_error_auditor;
use std::marker::PhantomData;
pub struct Formatter<S> {
    _subject: PhantomData<S>,
}
