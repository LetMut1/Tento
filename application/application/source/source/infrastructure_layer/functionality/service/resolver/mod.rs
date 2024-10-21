mod cloud_message;
mod unix_time;
mod expiration;
mod postgresql_transaction;
use std::marker::PhantomData;
pub use self::cloud_message::CloudMessage;
pub use self::unix_time::UnixTime;
pub use self::expiration::Expiration;
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
