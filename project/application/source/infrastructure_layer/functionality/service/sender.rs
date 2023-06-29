use std::marker::PhantomData;

pub use super::email___sender::Email;

pub struct Sender<S> {
    _subject: PhantomData<S>,
}
