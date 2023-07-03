use std::marker::PhantomData;

pub use super::unix_time___expiration_time_checker::UnixTime;

pub struct ExpirationTimeChecker<S> {
    _subject: PhantomData<S>,
}
