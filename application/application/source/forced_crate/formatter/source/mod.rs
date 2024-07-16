pub mod alternative_workflow;
mod backtrace;
mod external_auditor;
mod internal_error_auditor;
use std::marker::PhantomData;
pub struct Formatter<S> {
    _subject: PhantomData<S>,
}
