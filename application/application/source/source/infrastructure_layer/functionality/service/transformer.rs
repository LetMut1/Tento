use std::marker::PhantomData;

pub use super::http_request_body___transformer::Body;

pub struct Transformer<S> {
    _subject: PhantomData<S>,
}