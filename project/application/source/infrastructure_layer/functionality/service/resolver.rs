use std::marker::PhantomData;

pub use super::cloud_message_resolver::CloudMessage;
pub use super::date_time_resolver::DateTime;
pub use super::postgresql_transaction_resolver::PostgresqlTransaction;

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