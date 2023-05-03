use std::marker::PhantomData;

pub use super::request__validator::Request;

pub struct Validator<S> {
    _subject: PhantomData<S>
}