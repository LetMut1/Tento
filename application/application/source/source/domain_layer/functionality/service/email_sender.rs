use std::marker::PhantomData;

pub struct EmailSender<S> {
    _subject: PhantomData<S>,
}
