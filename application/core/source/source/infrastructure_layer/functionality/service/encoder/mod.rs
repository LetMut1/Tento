mod argon2id;
mod hmac;
mod highway;
pub use self::{
    argon2id::Argon2Id,
    hmac::HmacSha3_512,
    highway::Highway,
};
use std::marker::PhantomData;
pub struct Encoder<S> {
    _subject: PhantomData<S>,
}
