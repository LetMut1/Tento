mod number_row;
pub use self::number_row::NumberRow;
use std::marker::PhantomData;
pub struct Generator<S> {
    _subject: PhantomData<S>,
}
