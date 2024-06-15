use crate::Common1;
use common_precedent::CommonPrecedent;
use macro_rules::r#enum;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize)]
pub struct Incoming {
    pub application_user_access_token_encrypted: String,
    pub channel_name: String,
    pub requery_channel_name: Option<String>,
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
