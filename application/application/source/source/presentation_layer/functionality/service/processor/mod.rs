pub mod action_round;
use std::marker::PhantomData;
pub struct Processor<S> {
    _subject: PhantomData<S>,
}
