pub mod postgresql_connection_pool;
pub mod response;
pub mod router;
use std::marker::PhantomData;
pub struct Creator<S> {
    _subject: PhantomData<S>,
}
