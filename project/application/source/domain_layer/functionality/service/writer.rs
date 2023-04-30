use std::marker::PhantomData;

pub struct Writer<S> {
    _subject: PhantomData<S>
}