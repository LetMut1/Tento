use std::marker::PhantomData;

pub use super::postgresql_connection_pool__creator::PostgresqlConnectionPoolNoTls;
pub use super::redis_connection_pool__creator::RedisConnectonPool;
pub use super::response__creator::Response;

pub struct Creator<S> {
    _subject: PhantomData<S>,
}
