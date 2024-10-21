use user_access_refresh_token_encoded::UserAccessRefreshTokenEncoded;
use user_access_token_encoded::UserAccessTokenEncoded;
use common_precedent::CommonPrecedent;
use macro_rules::r#enum;
use serde::{
    Deserialize,
    Serialize,
};
use bitcode::{
    Encode,
    Decode,
};
#[derive(Serialize, Deserialize, Encode, Decode)]
pub struct Incoming {
    pub user_access_token_encoded: UserAccessTokenEncoded,
    pub user_access_refresh_token_encoded: UserAccessRefreshTokenEncoded,
}
#[derive(Serialize, Deserialize, Encode, Decode)]
pub struct Outcoming {
    pub user_access_token_encoded: UserAccessTokenEncoded,
    pub user_access_refresh_token_encoded: UserAccessRefreshTokenEncoded,
}
r#enum!(
    pub enum Precedent {
        CommonPrecedent::UserAccessRefreshToken_NotFound,
        CommonPrecedent::UserAccessRefreshToken_AlreadyExpired,
    }
);
