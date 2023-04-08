use std::marker::PhantomData;

pub struct ExpirationTimeResolver<S> {
    _subject: PhantomData<S>
}