use crate::Common1;
use common_precedent::CommonPrecedent;
use macro_rules::r#enum;
use serde::{
    Deserialize,
    Serialize,
};
#[derive(Serialize, Deserialize)]
pub struct Incoming {
    pub application_user_access_token_encrypted: String,
    pub channel__name: String,
    pub requery___channel__name: Option<String>,
    pub limit: i16,
}
#[derive(Serialize, Deserialize)]
pub struct Outcoming {
    pub common_registry: Vec<Common1>,
}
r#enum!(
    pub enum Precedent {
        CommonPrecedent::ApplicationUserAccessToken_AlreadyExpired,
        CommonPrecedent::ApplicationUserAccessToken_InApplicationUserAccessTokenBlackList,
    }
);
