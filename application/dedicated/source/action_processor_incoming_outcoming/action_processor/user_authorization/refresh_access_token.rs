use crate::{
    common_precedent::CommonPrecedent,
    user_access_refresh_token_signed::UserAccessRefreshTokenSigned,
    user_access_token_signed::{
        UserAccessTokenSigned,
        UserAccessTokenSigned_,
    },
};
#[cfg_attr(
    feature = "serde_for_manual_test",
    derive(
        serde::Serialize,
        serde::Deserialize
    )
)]
#[derive(bitcode::Encode, bitcode::Decode)]
pub struct Incoming<'a> {
    #[cfg_attr(feature = "serde_for_manual_test", serde(borrow))]
    pub user_access_token_signed: UserAccessTokenSigned<'a>,
    pub user_access_refresh_token_signed: UserAccessRefreshTokenSigned,
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
    pub user_access_token_signed: UserAccessTokenSigned_,
    pub user_access_refresh_token_signed: UserAccessRefreshTokenSigned,
}
crate::common_precedent::enum_from!(
    pub enum Precedent {
        CommonPrecedent::UserAccessRefreshToken_NotFound,
        CommonPrecedent::UserAccessRefreshToken_AlreadyExpired,
    }
);
