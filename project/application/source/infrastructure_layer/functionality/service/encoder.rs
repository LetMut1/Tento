use std::marker::PhantomData;

pub use super::argon2id__encoder::Argon2Id;
pub use super::base64__encoder::Base64;
pub use super::hmac__encoder::Hmac;

pub struct Encoder<S> {
    _subject: PhantomData<S>
}