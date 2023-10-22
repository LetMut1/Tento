pub mod invalid_argument;
pub mod error_auditor;
pub mod response;

use std::marker::PhantomData;

pub struct Reactor<S> {
    _subject: PhantomData<S>,
}