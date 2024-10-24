use crate::{
    common_precedent::CommonPrecedent,
    user_access_refresh_token_encoded::UserAccessRefreshTokenEncoded,
    user_access_token_encoded::UserAccessTokenEncoded,
};
#[cfg_attr(
    feature = "serde_for_manual_test",
    derive(
        serde::Serialize,
        serde::Deserialize
    )
)]
#[derive(bitcode::Encode, bitcode::Decode)]
pub struct Incoming {
    pub user_access_token_encoded: UserAccessTokenEncoded,
    pub user_access_refresh_token_encoded: UserAccessRefreshTokenEncoded,
}
#[cfg_attr(
    feature = "serde_for_manual_test",
    derive(
        serde::Serialize,
        serde::Deserialize
    )
)]
#[derive(bitcode::Encode, bitcode::Decode)]
pub struct Outcoming {
    pub user_access_token_encoded: UserAccessTokenEncoded,
    pub user_access_refresh_token_encoded: UserAccessRefreshTokenEncoded,
}
crate::macro_rules::enum_from!(
    pub enum Precedent {
        CommonPrecedent::UserAccessRefreshToken_NotFound,
        CommonPrecedent::UserAccessRefreshToken_AlreadyExpired,
    }
);
