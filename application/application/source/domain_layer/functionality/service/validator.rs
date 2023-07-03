use std::marker::PhantomData;

pub struct Validator<S> {
    _subject: PhantomData<S>,
}
