use application_user_access_refresh_token_encoded::UserAccessRefreshTokenEncoded;
use application_user_access_token_encoded::UserAccessTokenEncoded;
use common_precedent::CommonPrecedent;
use macro_rules::r#enum;
use serde::{
    Deserialize,
    Serialize,
};
#[derive(Serialize, Deserialize)]
pub struct Incoming {
    pub application_user_device__id: String,
    pub application_user__nickname: String,
    pub application_user__password: String,
    pub application_user__email: String,
    pub application_user_registration_token__value: String,
}
#[derive(Serialize, Deserialize)]
pub struct Outcoming {
    pub application_user_access_token_encoded: UserAccessTokenEncoded,
    pub application_user_access_refresh_token_encoded: UserAccessRefreshTokenEncoded,
}
r#enum!(
    pub enum Precedent {
        CommonPrecedent::User_NicknameAlreadyExist,
        CommonPrecedent::User_EmailAlreadyExist,
        CommonPrecedent::UserRegistrationToken_NotFound,
        CommonPrecedent::UserRegistrationToken_AlreadyExpired,
        CommonPrecedent::UserRegistrationToken_IsNotApproved,
        CommonPrecedent::UserRegistrationToken_WrongValue,
    }
);
