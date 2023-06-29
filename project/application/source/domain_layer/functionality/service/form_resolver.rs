use std::marker::PhantomData;

pub struct FormResolver<S> {
    _subject: PhantomData<S>,
}
