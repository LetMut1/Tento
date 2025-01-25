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
    pub user_device__id: String,
    pub user__nickname: String,
    pub user__password: String,
    pub user__email: String,
    pub user_registration_token__value: String,
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
crate::common_precedent::enum_from!(
    pub enum Precedent {
        CommonPrecedent::User_NicknameAlreadyExist,
        CommonPrecedent::User_EmailAlreadyExist,
        CommonPrecedent::UserRegistrationToken_NotFound,
        CommonPrecedent::UserRegistrationToken_AlreadyExpired,
        CommonPrecedent::UserRegistrationToken_IsNotApproved,
        CommonPrecedent::UserRegistrationToken_WrongValue,
    }
);
