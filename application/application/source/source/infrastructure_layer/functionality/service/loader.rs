use std::marker::PhantomData;

pub struct Loader<S> {
    _subject: PhantomData<S>,
}
