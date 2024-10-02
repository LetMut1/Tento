pub mod user_access_token;
use std::marker::PhantomData;
pub struct Extractor<S> {
    _subject: PhantomData<S>,
}
