use std::marker::PhantomData;

pub struct Formatter<S> {
    _subject: PhantomData<S>,
}

pub trait Format<T> {
    fn prepare<'a>(subject: &'a T) -> String;
}