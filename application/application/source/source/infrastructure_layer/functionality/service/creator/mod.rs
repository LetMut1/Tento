pub mod postgresql_connection_pool;
pub mod redis_connection_pool;
pub mod response;

use std::marker::PhantomData;

pub struct Creator<S> {
    _subject: PhantomData<S>,
}
