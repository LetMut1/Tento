use common_precedent::CommonPrecedent;
use macro_rules::r#enum;
use serde::{
    Deserialize,
    Serialize,
};
#[derive(Serialize, Deserialize)]
pub struct Incoming {
    pub application_user_access_token_encrypted: String,
    pub application_user_access_refresh_token_encrypted: String,
}
#[derive(Serialize, Deserialize)]
pub struct Outcoming {
    pub application_user_access_token_encrypted: String,
    pub application_user_access_refresh_token_encrypted: String,
}
r#enum!(
    pub enum Precedent {
        CommonPrecedent::ApplicationUserAccessRefreshToken_NotFound,
        CommonPrecedent::ApplicationUserAccessRefreshToken_AlreadyExpired,
    }
);
