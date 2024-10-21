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
    pub user_device__id: String,
    pub user__nickname: String,
    pub user__password: String,
    pub user__email: String,
    pub user_registration_token__value: String,
}
#[derive(Serialize, Deserialize, Encode, Decode)]
pub struct Outcoming {
    pub user_access_token_encoded: UserAccessTokenEncoded,
    pub user_access_refresh_token_encoded: UserAccessRefreshTokenEncoded,
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
