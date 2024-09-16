use common_precedent::CommonPrecedent;
use macro_rules::r#enum;
use serde::{
    Deserialize,
    Serialize,
};
use application_user_access_token_encrypted::ApplicationUserAccessTokenEncrypted;
#[derive(Serialize, Deserialize)]
pub struct Incoming {
    pub application_user__id: i64,
    pub application_user_device__id: String,
    pub application_user_authorization_token__value: String,
}
#[derive(Serialize, Deserialize)]
pub struct Outcoming {
    pub application_user_access_token_encrypted: ApplicationUserAccessTokenEncrypted,
    pub application_user_access_refresh_token_encrypted: Vec<u8>,
}
r#enum!(
    pub enum Precedent {
        CommonPrecedent::ApplicationUserAuthorizationToken_NotFound,
        CommonPrecedent::ApplicationUserAuthorizationToken_AlreadyExpired,
        CommonPrecedent::ApplicationUserAuthorizationToken_WrongValue {
            application_user_authorization_token__wrong_enter_tries_quantity: i16,
        },
        CommonPrecedent::ApplicationUser_NotFound,
    }
);
