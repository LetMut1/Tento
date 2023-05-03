use std::marker::PhantomData;

pub use super::cloud_message_resolver::CloudMessage;

pub struct Resolver<S> {
    _subject: PhantomData<S>
}