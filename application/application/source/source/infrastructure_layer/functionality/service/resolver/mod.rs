mod cloud_message;
mod expiration;
mod postgresql_transaction;
mod unix_time;
pub use self::{
    cloud_message::CloudMessage,
    expiration::Expiration,
    unix_time::UnixTime,
};
use std::marker::PhantomData;
pub struct Resolver<S> {
    _subject: PhantomData<S>,
}
impl<S> Resolver<S> {
    pub fn new() -> Self {
        return Self {
            _subject: PhantomData,
        };
    }
}
