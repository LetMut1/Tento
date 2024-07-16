pub mod action_round___external_auditor;
pub mod action_round___internal_error_auditor;
pub mod action_round___response;
pub mod alternative_workflow;
use std::marker::PhantomData;
pub struct Logger<S> {
    _subject: PhantomData<S>,
}
