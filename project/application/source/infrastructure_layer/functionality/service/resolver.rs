use std::marker::PhantomData;

pub use super::cloud_message_resolver::CloudMessage;
pub use super::date_time_resolver::DateTime;

pub struct Resolver<S> {
    _subject: PhantomData<S>
}