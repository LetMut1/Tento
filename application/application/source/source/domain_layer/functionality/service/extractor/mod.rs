mod user_access_token;
pub use self::user_access_token::Extracted;
use std::marker::PhantomData;
pub struct Extractor<S> {
    _subject: PhantomData<S>,
}
