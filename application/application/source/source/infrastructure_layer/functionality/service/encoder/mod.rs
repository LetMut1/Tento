mod argon2id;
mod hmac;
use std::marker::PhantomData;
pub use self::argon2id::Argon2Id;
pub use self::hmac::HmacSha3_512;
pub struct Encoder<S> {
    _subject: PhantomData<S>,
}
