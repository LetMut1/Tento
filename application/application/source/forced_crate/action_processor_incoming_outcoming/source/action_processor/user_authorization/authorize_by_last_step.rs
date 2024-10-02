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
    pub application_user__id: i64,
    pub application_user_device__id: String,
    pub application_user_authorization_token__value: String,
}
#[derive(Serialize, Deserialize)]
pub struct Outcoming {
    pub application_user_access_token_encoded: UserAccessTokenEncoded,
    pub application_user_access_refresh_token_encoded: UserAccessRefreshTokenEncoded,
}
r#enum!(
    pub enum Precedent {
        CommonPrecedent::UserAuthorizationToken_NotFound,
        CommonPrecedent::UserAuthorizationToken_AlreadyExpired,
        CommonPrecedent::UserAuthorizationToken_WrongValue {
            application_user_authorization_token__wrong_enter_tries_quantity: i16,
        },
        CommonPrecedent::User_NotFound,
    }
);
