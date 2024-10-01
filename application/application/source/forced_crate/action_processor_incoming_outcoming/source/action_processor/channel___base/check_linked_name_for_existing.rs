use application_user_access_token_encoded::ApplicationUserAccessTokenEncoded;
use serde::{
    Deserialize,
    Serialize,
};
use common_precedent::CommonPrecedent;
use macro_rules::r#enum;
#[derive(Serialize, Deserialize)]
pub struct Incoming {
    pub application_user_access_token_encoded: ApplicationUserAccessTokenEncoded,
    pub channel__linked_name: String,
}
#[derive(Serialize, Deserialize)]
pub struct Outcoming {
    pub result: bool,
}
r#enum!(
    pub enum Precedent {
        CommonPrecedent::ApplicationUserAccessToken_AlreadyExpired,
        CommonPrecedent::ApplicationUserAccessToken_InApplicationUserAccessTokenBlackList,
    }
);
