use common_precedent::CommonPrecedent;
use macro_rules::r#enum;
use serde::{
    Deserialize,
    Serialize,
};
use application_user_access_refresh_token_encrypted::ApplicationUserAccessRefreshTokenEncrypted;
use application_user_access_token_encrypted::ApplicationUserAccessTokenEncrypted;
#[derive(Serialize, Deserialize)]
pub struct Incoming {
    pub application_user_access_token_encrypted: ApplicationUserAccessTokenEncrypted,
    pub application_user_access_refresh_token_encrypted: ApplicationUserAccessRefreshTokenEncrypted,
}
#[derive(Serialize, Deserialize)]
pub struct Outcoming {
    pub application_user_access_token_encrypted: ApplicationUserAccessTokenEncrypted,
    pub application_user_access_refresh_token_encrypted: ApplicationUserAccessRefreshTokenEncrypted,
}
r#enum!(
    pub enum Precedent {
        CommonPrecedent::ApplicationUserAccessRefreshToken_NotFound,
        CommonPrecedent::ApplicationUserAccessRefreshToken_AlreadyExpired,
    }
);
