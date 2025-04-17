mod number_row;
#[cfg(not(feature = "token_value_666666"))]
pub use self::number_row::NumberRow;
use std::marker::PhantomData;
pub struct Generator<S> {
    _subject: PhantomData<S>,
}
