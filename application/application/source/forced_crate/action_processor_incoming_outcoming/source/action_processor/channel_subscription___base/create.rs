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
    channel_id: Channel_Id,
}