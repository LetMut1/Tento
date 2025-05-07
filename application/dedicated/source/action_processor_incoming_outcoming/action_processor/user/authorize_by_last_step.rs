use crate::{
    common_precedent::CommonPrecedent,
    user_access_refresh_token_signed::UserAccessRefreshTokenSigned,
    user_access_token_signed::UserAccessTokenSigned_,
};
#[cfg_attr(
    feature = "serde_for_manual_test",
    derive(
        serde::Serialize,
        serde::Deserialize,
    )
)]
#[derive(bitcode::Encode, bitcode::Decode)]
pub struct Incoming<'a> {
    pub user__obfuscated_id: i64,
    pub user_device__id: &'a str,
    pub user_authorization_token__value: &'a str,
}
#[cfg_attr(
    feature = "serde_for_manual_test",
    derive(
        serde::Serialize,
        serde::Deserialize,
    )
)]
#[derive(bitcode::Encode, bitcode::Decode)]
pub struct Outcoming {
    pub user_access_token_signed: UserAccessTokenSigned_,
    pub user_access_refresh_token_signed: UserAccessRefreshTokenSigned,
}
crate::common_precedent::enum_from!(
    pub enum Precedent {
        CommonPrecedent::UserAuthorizationToken__NotFound,
        CommonPrecedent::UserAuthorizationToken__AlreadyExpired,
        CommonPrecedent::UserAuthorizationToken__WrongValue {
            user_authorization_token__wrong_enter_tries_quantity: u8,
        },
        CommonPrecedent::User__NotFound,
        CommonPrecedent::ParallelExecution,
    }
);
