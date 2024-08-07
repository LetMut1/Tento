use bb8::Pool;
use bb8_postgres::PostgresConnectionManager as PostgresqlConnectionManager;
use hyper::{
    Body,
    Request as HyperRequest,
    Response as HyperResponse,
};
use tokio_postgres::NoTls;
pub struct ApplicationUser__Authorization___AuthorizeByFirstStep;
pub struct ApplicationUser__Authorization___AuthorizeByLastStep;
pub struct ApplicationUser__Authorization___CheckEmailForExisting;
pub struct ApplicationUser__Authorization___CheckNicknameForExisting;
pub struct ApplicationUser__Authorization___DeauthorizeFromAllDevices;
pub struct ApplicationUser__Authorization___DeauthorizeFromOneDevice;
pub struct ApplicationUser__Authorization___RefreshAccessToken;
pub struct ApplicationUser__Authorization___RegisterByFirstStep;
pub struct ApplicationUser__Authorization___RegisterByLastStep;
pub struct ApplicationUser__Authorization___RegisterBySecondStep;
pub struct ApplicationUser__Authorization___ResetPasswordByFirstStep;
pub struct ApplicationUser__Authorization___ResetPasswordByLastStep;
pub struct ApplicationUser__Authorization___ResetPasswordBySecondStep;
pub struct ApplicationUser__Authorization___SendEmailForAuthorize;
pub struct ApplicationUser__Authorization___SendEmailForRegister;
pub struct ApplicationUser__Authorization___SendEmailForResetPassword;
pub struct Argon2Id;
pub struct Base64;
pub struct Channel__Base___GetManyByNameInSubscriptions;
pub struct Channel__Base___GetManyBySubscription;
pub struct Channel__Base___GetManyPublicByName;
pub struct Channel__Base___GetOneById;
pub struct ChannelSubscription__Base___Create;
pub struct CloudMessage;
pub struct CreateFixtures;
pub struct DateTime;
pub struct Email;
pub struct RemoveIncompliteState;
pub struct HttpBody;
pub struct ActionRound;
pub struct HealthCheck;
pub struct MessagePack;
pub struct NumberRow;
pub struct PostgresqlTransaction;
pub struct RouteNotFound;
pub struct RunServer;
pub struct TokioBlockingTask;
pub struct TokioNonBlockingTask;
pub struct UnixTime;
pub type PostgresqlConnectionPoolNoTls = Pool<PostgresqlConnectionManager<NoTls>>;
pub type Request = HyperRequest<Body>;
pub type Response = HyperResponse<Body>;
#[cfg(feature = "manual_testing")]
pub struct Json;
