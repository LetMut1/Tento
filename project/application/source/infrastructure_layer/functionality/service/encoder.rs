use std::marker::PhantomData;

pub use super::argon2id_encoder::Argon2Id;
pub use super::base64_encoder::Base64;
pub use super::hmac_encoder::Hmac;

pub struct Encoder<S> {
    _subject: PhantomData<S>
}