pub mod action_round;
pub mod aggregate_error;
use std::marker::PhantomData;
pub struct Logger<S> {
    _subject: PhantomData<S>,
}
