use std::marker::PhantomData;

pub use super::request___validator::Request;

pub struct Validator<S> {
    _subject: PhantomData<S>,
}
