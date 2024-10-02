use user_access_refresh_token_encoded::UserAccessRefreshTokenEncoded;
use user_access_token_encoded::UserAccessTokenEncoded;
use common_precedent::CommonPrecedent;
use macro_rules::r#enum;
use serde::{
    Deserialize,
    Serialize,
};
#[derive(Serialize, Deserialize)]
pub struct Incoming {
    pub application_user_access_token_encoded: UserAccessTokenEncoded,
    pub application_user_access_refresh_token_encoded: UserAccessRefreshTokenEncoded,
}
#[derive(Serialize, Deserialize)]
pub struct Outcoming {
    pub application_user_access_token_encoded: UserAccessTokenEncoded,
    pub application_user_access_refresh_token_encoded: UserAccessRefreshTokenEncoded,
}
r#enum!(
    pub enum Precedent {
        CommonPrecedent::UserAccessRefreshToken_NotFound,
        CommonPrecedent::UserAccessRefreshToken_AlreadyExpired,
    }
);
