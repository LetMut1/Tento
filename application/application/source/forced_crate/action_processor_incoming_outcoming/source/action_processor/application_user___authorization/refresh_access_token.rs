use common_precedent::CommonPrecedent;
use macro_rules::r#enum;
use serde::{
    Deserialize,
    Serialize,
};
use application_user_access_refresh_token_encoded::ApplicationUserAccessRefreshTokenEncoded;
use application_user_access_token_encoded::ApplicationUserAccessTokenEncoded;
#[derive(Serialize, Deserialize)]
pub struct Incoming {
    pub application_user_access_token_encoded: ApplicationUserAccessTokenEncoded,
    pub application_user_access_refresh_token_encoded: ApplicationUserAccessRefreshTokenEncoded,
}
#[derive(Serialize, Deserialize)]
pub struct Outcoming {
    pub application_user_access_token_encoded: ApplicationUserAccessTokenEncoded,
    pub application_user_access_refresh_token_encoded: ApplicationUserAccessRefreshTokenEncoded,
}
r#enum!(
    pub enum Precedent {
        CommonPrecedent::ApplicationUserAccessRefreshToken_NotFound,
        CommonPrecedent::ApplicationUserAccessRefreshToken_AlreadyExpired,
    }
);
