pub mod argon2id;
pub mod hmac;
use std::marker::PhantomData;
pub struct Encoder<S> {
    _subject: PhantomData<S>,
}
