use std::marker::PhantomData;

pub struct Incrementor<S> {
    _subject: PhantomData<S>,
}
