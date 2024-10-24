use user_access_refresh_token_encoded::UserAccessRefreshTokenEncoded;
use user_access_token_encoded::UserAccessTokenEncoded;
use common_precedent::CommonPrecedent;
#[cfg_attr(feature = "serde_for_manual_test", derive(serde::Serialize, serde::Deserialize))]
#[derive(bitcode::Encode, bitcode::Decode)]
pub struct Incoming {
    pub user_device__id: String,
    pub user__nickname: String,
    pub user__password: String,
    pub user__email: String,
    pub user_registration_token__value: String,
}
#[cfg_attr(feature = "serde_for_manual_test", derive(serde::Serialize, serde::Deserialize))]
#[derive(bitcode::Encode, bitcode::Decode)]
pub struct Outcoming {
    pub user_access_token_encoded: UserAccessTokenEncoded,
    pub user_access_refresh_token_encoded: UserAccessRefreshTokenEncoded,
}
macro_rules::enum_from!(
    pub enum Precedent {
        CommonPrecedent::User_NicknameAlreadyExist,
        CommonPrecedent::User_EmailAlreadyExist,
        CommonPrecedent::UserRegistrationToken_NotFound,
        CommonPrecedent::UserRegistrationToken_AlreadyExpired,
        CommonPrecedent::UserRegistrationToken_IsNotApproved,
        CommonPrecedent::UserRegistrationToken_WrongValue,
    }
);
