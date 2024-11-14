mod cloud_message;
mod unix_time;
pub use self::{
    cloud_message::CloudMessage,
    unix_time::UnixTime,
};
use std::marker::PhantomData;
pub struct Resolver<S> {
    _subject: PhantomData<S>,
}
