mod action_round;
mod responsive;
mod unresponsive;
use std::marker::PhantomData;
pub use self::action_round::RowData;
pub struct Formatter<S> {
    _subject: PhantomData<S>,
}
pub trait Format<S> {
    fn format<'a>(subject: &'a S) -> String;
}
