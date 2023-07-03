use std::marker::PhantomData;

pub struct Encoder<S> {
    _subject: PhantomData<S>,
}
