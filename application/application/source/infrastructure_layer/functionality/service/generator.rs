use std::marker::PhantomData;

pub use super::number_row___generator::NumberRow;

pub struct Generator<S> {
    _subject: PhantomData<S>,
}
