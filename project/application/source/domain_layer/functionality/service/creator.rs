use std::marker::PhantomData;

pub struct Creator<S> {
    _subject: PhantomData<S>
}