use crate::Common1;
use entity::application_user_access_token_encrypted::ApplicationUserAccessTokenEncrypted;
use entity::channel::Channel_Id;
use serde::Deserialize;
use serde::Serialize;

#[cfg_attr(
    feature = "manual_testing",
    derive(Serialize)
)]
#[derive(Deserialize)]
pub struct Incoming {
    application_user_access_token_encrypted: ApplicationUserAccessTokenEncrypted,
    requery_channel_id: Option<Channel_Id>,
    limit: i16,
}

#[cfg_attr(
    feature = "manual_testing",
    derive(Deserialize)
)]
#[derive(Serialize)]
pub struct Outcoming {
    common_registry: Vec<Common1>,
}