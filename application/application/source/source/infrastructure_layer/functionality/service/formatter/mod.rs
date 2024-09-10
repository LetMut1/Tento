pub mod action_round;
pub mod responsive;
pub mod unresponsive;
use std::marker::PhantomData;
pub struct Formatter<S> {
    _subject: PhantomData<S>,
}
pub trait Format<S> {
    fn format<'a>(subject: &'a S) -> String;
}
