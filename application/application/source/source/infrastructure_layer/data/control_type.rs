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

pub struct RegisterByLastStep;

pub struct ResetPasswordByLastStep;

pub struct SendEmailForResetPassword;

pub struct RouteNotFound;

pub struct GetManyPublicByName;

pub struct ResetPasswordByFirstStep;

pub struct Create;

pub struct GetManyBySubscription;

pub struct DateTime;

pub struct GetOneByID;

pub struct AuthorizeByLastStep;

pub struct RegisterByFirstStep;

pub struct RefreshAccessToken;

pub struct ResetPasswordBySecondStep;

pub struct UnixTime;

pub struct DeauthorizeFromOneDevice;

pub struct CheckNicknameForExisting;

pub struct DeauthorizeFromAllDevices;

pub struct CheckEmailForExisting;

pub struct SendEmailForAuthorize;

pub struct Action;

pub struct SendEmailForRegister;

pub struct Email;

pub struct HttpBodyData;

pub struct AuthorizeByFirstStep;

pub struct HealthCheck;

pub struct RegisterBySecondStep;

pub struct MessagePack;

pub struct GetManyByNameInSubscriptions;

pub struct NumberRow;

pub struct PostgresqlTransaction;

pub type PostgresqlConnectionPoolNoTls = Pool<PostgresqlConnectionManager<NoTls>>;

pub type RedisConnectonPool = Pool<RedisConnectionManager>;

pub type Request = HyperRequest<Body>;

pub type Response = HyperResponse<Body>;

#[cfg(feature = "manual_testing")]
pub struct Json;
