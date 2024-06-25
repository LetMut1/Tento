mod backtrace;
mod internal_auditor;
mod external_auditor;
pub mod alternative_workflow;
use std::marker::PhantomData;
pub struct Formatter<S> {
    _subject: PhantomData<S>,
}
