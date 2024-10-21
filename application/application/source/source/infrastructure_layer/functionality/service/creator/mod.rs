mod postgresql_connection_pool;
mod response;
use std::marker::PhantomData;
pub struct Creator<S> {
    _subject: PhantomData<S>,
}
