use std::marker::PhantomData;

pub use super::email_sender::Email;

pub struct Sender<S> {
    _subject: PhantomData<S>
}