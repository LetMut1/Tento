use crate::Common1;
use entity::application_user_access_token_encrypted::ApplicationUserAccessTokenEncrypted;
use entity::channel::Channel_Name;
use serde::Deserialize;
use serde::Serialize;
use macro_rules::r#enum;
use common_precedent::CommonPrecedent;

#[derive(Serialize, Deserialize)]
pub struct Incoming {
    pub application_user_access_token_encrypted: ApplicationUserAccessTokenEncrypted,
    pub channel_name: Channel_Name,
    pub requery_channel_name: Option<Channel_Name>,
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