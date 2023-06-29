use std::marker::PhantomData;

pub use super::cloud_message___resolver::CloudMessage;
pub use super::date_time___resolver::DateTime;
pub use super::postgresql_transaction___resolver::PostgresqlTransaction;

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
