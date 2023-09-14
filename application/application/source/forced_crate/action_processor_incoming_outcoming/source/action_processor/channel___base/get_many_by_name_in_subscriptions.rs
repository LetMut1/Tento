use crate::Common1;
use entity::application_user_access_token_encrypted::ApplicationUserAccessTokenEncrypted;
use entity::channel::Channel_Name;
use serde::Deserialize;
use serde::Serialize;

#[cfg_attr(
    feature = "manual_testing",
    derive(Serialize)
)]
#[derive(Deserialize)]
pub struct Incoming {
    pub application_user_access_token_encrypted: ApplicationUserAccessTokenEncrypted,
    pub channel_name: Channel_Name,
    pub requery_channel_name: Option<Channel_Name>,
    pub limit: i16,
}

#[cfg_attr(
    feature = "manual_testing",
    derive(Deserialize)
)]
#[derive(Serialize)]
pub struct Outcoming {
    pub common_registry: Vec<Common1>,
}