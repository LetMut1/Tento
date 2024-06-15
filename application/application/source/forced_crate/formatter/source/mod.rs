mod backtrace;
mod error_auditor;
use std::marker::PhantomData;
pub struct Formatter<S> {
    _subject: PhantomData<S>,
}
