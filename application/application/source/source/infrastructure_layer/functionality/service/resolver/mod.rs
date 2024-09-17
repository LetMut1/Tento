pub mod cloud_message;
pub mod date_time;
pub mod postgresql_transaction;
pub mod expiration;
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
