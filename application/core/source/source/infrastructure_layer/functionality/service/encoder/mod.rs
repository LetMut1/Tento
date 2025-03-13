mod argon2id;
mod hmac_sha3_512;
mod hmac_sha2_256;
mod highway;
pub use self::{
    argon2id::Argon2Id,
    hmac_sha3_512::HmacSha3_512,
    highway::Highway,
};
use std::marker::PhantomData;
pub struct Encoder<S> {
    _subject: PhantomData<S>,
}
