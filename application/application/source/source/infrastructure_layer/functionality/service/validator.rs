use std::marker::PhantomData;

pub use super::request_parts___validator::Parts;

pub struct Validator<S> {
    _subject: PhantomData<S>,
}
