use std::marker::PhantomData;

pub use super::action___processor::Action;

pub struct Processor<S> {
    _subject: PhantomData<S>,
}
