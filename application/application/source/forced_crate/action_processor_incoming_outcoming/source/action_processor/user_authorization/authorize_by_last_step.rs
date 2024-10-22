use user_access_refresh_token_encoded::UserAccessRefreshTokenEncoded;
use user_access_token_encoded::UserAccessTokenEncoded;
use common_precedent::CommonPrecedent;
use macro_rules::enum_from;
#[cfg_attr(feature = "serde_for_manual_test", derive(serde::Serialize, serde::Deserialize))]
#[derive(bitcode::Encode, bitcode::Decode)]
pub struct Incoming {
    pub user__id: i64,
    pub user_device__id: String,
    pub user_authorization_token__value: String,
}
#[cfg_attr(feature = "serde_for_manual_test", derive(serde::Serialize, serde::Deserialize))]
#[derive(bitcode::Encode, bitcode::Decode)]
pub struct Outcoming {
    pub user_access_token_encoded: UserAccessTokenEncoded,
    pub user_access_refresh_token_encoded: UserAccessRefreshTokenEncoded,
}
enum_from!(
    pub enum Precedent {
        CommonPrecedent::UserAuthorizationToken_NotFound,
        CommonPrecedent::UserAuthorizationToken_AlreadyExpired,
        CommonPrecedent::UserAuthorizationToken_WrongValue {
            user_authorization_token__wrong_enter_tries_quantity: i16,
        },
        CommonPrecedent::User_NotFound,
    }
);
