use std::marker::PhantomData;

pub use super::http_body_data___extractor::HttpBodyData;

pub struct Extractor<S> {
    _subject: PhantomData<S>,
}
