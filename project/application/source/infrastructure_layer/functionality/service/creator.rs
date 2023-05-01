use std::marker::PhantomData;
pub use super::postgresql_connection_pool_creator::PostgresqlConnectionPoolNoTls;
pub use super::redis_connection_pool_creator::RedisConnectonPool;

pub struct Creator<S> {
    _subject: PhantomData<S>
}