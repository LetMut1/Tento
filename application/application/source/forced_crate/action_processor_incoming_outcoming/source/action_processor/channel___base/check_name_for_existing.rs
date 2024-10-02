use user_access_token_encoded::UserAccessTokenEncoded;
use serde::{
    Deserialize,
    Serialize,
};
use common_precedent::CommonPrecedent;
use macro_rules::r#enum;
#[derive(Serialize, Deserialize)]
pub struct Incoming {
    pub user_access_token_encoded: UserAccessTokenEncoded,
    pub channel__name: String,
}
#[derive(Serialize, Deserialize)]
pub struct Outcoming {
    pub result: bool,
}
r#enum!(
    pub enum Precedent {
        CommonPrecedent::UserAccessToken_AlreadyExpired,
        CommonPrecedent::UserAccessToken_InUserAccessTokenBlackList,
    }
);
