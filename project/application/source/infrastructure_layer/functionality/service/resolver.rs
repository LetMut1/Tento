use std::marker::PhantomData;

pub use super::cloud_message__resolver::CloudMessage;
pub use super::date_time__resolver::DateTime;
pub use super::postgresql_transaction__resolver::PostgresqlTransaction;

pub struct Resolver<S> {
    _subject: PhantomData<S>
}

impl<S> Resolver<S> {
    pub fn new() -> Self {
        return Self {
            _subject: PhantomData
        };
    }
}