use std::marker::PhantomData;

pub use super::email__sender::Email;

pub struct Sender<S> {
    _subject: PhantomData<S>
}