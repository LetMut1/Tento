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
    pub user__id: i64,
    pub user_device__id: String,
    pub user_authorization_token__value: String,
}
#[derive(Serialize, Deserialize)]
pub struct Outcoming {
    pub user_access_token_encoded: UserAccessTokenEncoded,
    pub user_access_refresh_token_encoded: UserAccessRefreshTokenEncoded,
}
r#enum!(
    pub enum Precedent {
        CommonPrecedent::UserAuthorizationToken_NotFound,
        CommonPrecedent::UserAuthorizationToken_AlreadyExpired,
        CommonPrecedent::UserAuthorizationToken_WrongValue {
            user_authorization_token__wrong_enter_tries_quantity: i16,
        },
        CommonPrecedent::User_NotFound,
    }
);
