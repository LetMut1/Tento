mod argon2id;
mod highway;
mod hmac_sha2_256;
mod hmac_sha3_512;
pub use self::{
    argon2id::Argon2Id,
    highway::Highway,
    hmac_sha3_512::HmacSha3_512,
};
use std::marker::PhantomData;
pub struct Encoder<S> {
    _subject: PhantomData<S>,
}
