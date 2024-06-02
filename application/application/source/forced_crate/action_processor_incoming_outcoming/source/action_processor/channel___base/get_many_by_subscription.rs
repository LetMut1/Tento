use crate::Common1;
use serde::Deserialize;
use serde::Serialize;
use macro_rules::r#enum;
use common_precedent::CommonPrecedent;

#[derive(Serialize, Deserialize)]
pub struct Incoming {
    pub application_user_access_token_encrypted: String,
    pub requery_channel_id: Option<i64>,
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