use bb8::Pool;
use bb8_postgres::PostgresConnectionManager as PostgresqlConnectionManager;
use bb8_redis::RedisConnectionManager;
use hyper::Body;
use hyper::Request as HyperRequest;
use hyper::Response as HyperResponse;
use tokio_postgres::NoTls;

pub struct Argon2Id;

pub struct Base64;

pub struct CloudMessage;

pub struct DateTime;

pub struct UnixTime;

pub struct Action;

pub struct Email;

pub struct HttpBodyData;

pub struct MessagePack;

pub struct NumberRow;

pub struct PostgresqlTransaction;

pub type PostgresqlConnectionPoolNoTls = Pool<PostgresqlConnectionManager<NoTls>>;

pub type RedisConnectonPool = Pool<RedisConnectionManager>;

pub type Request = HyperRequest<Body>;

pub type Response = HyperResponse<Body>;

#[cfg(feature = "manual_testing")]
pub struct Json;
