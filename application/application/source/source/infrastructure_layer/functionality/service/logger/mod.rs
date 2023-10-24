pub mod action_round_log___invalid_argument;
pub mod action_round_log___error_auditor;
pub mod action_round_log___response;
pub mod error_auditor;

use std::marker::PhantomData;

pub struct Logger<S> {
    _subject: PhantomData<S>,
}