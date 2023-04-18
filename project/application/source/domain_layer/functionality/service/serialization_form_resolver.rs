use std::marker::PhantomData;

pub struct SerializationFormResolver<S> {
    _subject: PhantomData<S>
}