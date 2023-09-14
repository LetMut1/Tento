use entity::application_user_access_refresh_token_encrypted::ApplicationUserAccessRefreshTokenEncrypted;
use entity::application_user_access_token_encrypted::ApplicationUserAccessTokenEncrypted;
use serde::Deserialize;
use serde::Serialize;

#[cfg_attr(
    feature = "manual_testing",
    derive(Serialize)
)]
#[derive(Deserialize)]
pub struct Incoming {
    pub application_user_access_token_encrypted: ApplicationUserAccessTokenEncrypted,
    pub application_user_access_refresh_token_encrypted: ApplicationUserAccessRefreshTokenEncrypted,
}

#[cfg_attr(
    feature = "manual_testing",
    derive(Deserialize)
)]
#[derive(Serialize)]
pub struct Outcoming {
    pub application_user_access_token_encrypted: ApplicationUserAccessTokenEncrypted,
    pub application_user_access_refresh_token_encrypted: ApplicationUserAccessRefreshTokenEncrypted,
}