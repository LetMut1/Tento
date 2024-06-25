pub mod action_round___external_auditor;
pub mod action_round___response;
pub mod error;
pub mod action_round___internal_auditor;
use std::marker::PhantomData;
pub struct Logger<S> {
    _subject: PhantomData<S>,
}
