use std::marker::PhantomData;

pub use super::argon2id___encoder::Argon2Id;
pub use super::base64___encoder::Base64;
pub use super::hmac___encoder::Hmac;

pub struct Encoder<S> {
    _subject: PhantomData<S>,
}
