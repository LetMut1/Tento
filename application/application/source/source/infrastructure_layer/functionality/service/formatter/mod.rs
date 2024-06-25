pub mod action_round___external_auditor;
pub mod action_round___internal_auditor;
pub mod action_round___response;
pub mod alternative_workflow;
pub mod backtrace;
pub mod external_auditor;
pub mod internal_auditor;
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
