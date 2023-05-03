use extern_crate::bb8_postgres::PostgresConnectionManager as PostgresqlConnectionManager;
use extern_crate::bb8_redis::RedisConnectionManager;
use extern_crate::bb8::Pool;
use extern_crate::hyper::Body;
use extern_crate::hyper::Request as HyperRequest;
use extern_crate::hyper::Response as HyperResponse;
use extern_crate::tokio_postgres::NoTls;

pub struct Argon2Id;

pub struct Base64;

pub struct Hmac;

pub struct CloudMessage;

pub struct DateTime;

pub struct Email;

pub struct MessagePack;

pub struct NumberRow;

pub struct PostgresqlTransaction;

pub type PostgresqlConnectionPoolNoTls = Pool<PostgresqlConnectionManager<NoTls>>;

pub type RedisConnectonPool = Pool<RedisConnectionManager>;

pub type Request = HyperRequest<Body>;

pub type Response = HyperResponse<Body>;

#[cfg(feature = "facilitate_non_automatic_functional_testing")]
pub struct Json;