pub mod invalid_argument;
pub mod error_auditor;

use std::marker::PhantomData;

pub struct Reactor<S> {
    _subject: PhantomData<S>,
}