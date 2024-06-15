pub mod number_row;
use std::marker::PhantomData;
pub struct Generator<S> {
    _subject: PhantomData<S>,
}
