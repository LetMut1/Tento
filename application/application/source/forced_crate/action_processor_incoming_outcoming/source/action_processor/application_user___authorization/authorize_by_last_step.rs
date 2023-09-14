use entity::application_user::ApplicationUser_Id;
use entity::application_user_access_refresh_token_encrypted::ApplicationUserAccessRefreshTokenEncrypted;
use entity::application_user_access_token_encrypted::ApplicationUserAccessTokenEncrypted;
use entity::application_user_authorization_token::ApplicationUserAuthorizationToken_Value;
use entity::application_user_device::ApplicationUserDevice_Id;
use serde::Deserialize;
use serde::Serialize;

#[cfg_attr(
    feature = "manual_testing",
    derive(Serialize)
)]
#[derive(Deserialize)]
pub struct Incoming {
    pub application_user_id: ApplicationUser_Id,
    pub application_user_device_id: ApplicationUserDevice_Id,
    pub application_user_authorization_token_value: ApplicationUserAuthorizationToken_Value,
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