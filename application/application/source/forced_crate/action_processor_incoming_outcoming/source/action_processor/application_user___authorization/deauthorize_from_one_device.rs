use serde::Deserialize;
use serde::Serialize;
use macro_rules::r#enum;
use common_precedent::CommonPrecedent;

#[derive(Serialize, Deserialize)]
pub struct Incoming {
    pub application_user_access_token_encrypted: String,
}

r#enum!(
    pub enum Precedent {
        CommonPrecedent::ApplicationUserAccessToken_AlreadyExpired,
        CommonPrecedent::ApplicationUserAccessToken_InApplicationUserAccessTokenBlackList,
    }
);