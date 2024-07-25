pub mod application_user___authorization;
pub mod channel___base;
pub mod channel_subscription___base;
pub mod health_check;
use std::marker::PhantomData;
pub struct ActionProcessor<S> {
    _subject: PhantomData<S>,
}
