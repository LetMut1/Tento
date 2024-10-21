mod http_request_parts;
use std::marker::PhantomData;
pub struct Validator<S> {
    _subject: PhantomData<S>,
}
