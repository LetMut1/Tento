use entity::application_user::ApplicationUser_Email;
use entity::application_user::ApplicationUser_Nickname;
use entity::application_user::ApplicationUser_Password;
use entity::application_user_access_refresh_token_encrypted::ApplicationUserAccessRefreshTokenEncrypted;
use entity::application_user_access_token_encrypted::ApplicationUserAccessTokenEncrypted;
use entity::application_user_device::ApplicationUserDevice_Id;
use entity::application_user_registration_token::ApplicationUserRegistrationToken_Value;
use serde::Deserialize;
use serde::Serialize;

#[cfg_attr(
    feature = "manual_testing",
    derive(Serialize)
)]
#[derive(Deserialize)]
pub struct Incoming {
    application_user_device_id: ApplicationUserDevice_Id,
    application_user_nickname: ApplicationUser_Nickname,
    application_user_password: ApplicationUser_Password,
    application_user_email: ApplicationUser_Email,
    application_user_registration_token_value: ApplicationUserRegistrationToken_Value,
}

#[cfg_attr(
    feature = "manual_testing",
    derive(Deserialize)
)]
#[derive(Serialize)]
pub struct Outcoming {
    application_user_access_token_encrypted: ApplicationUserAccessTokenEncrypted,
    application_user_access_refresh_token_encrypted: ApplicationUserAccessRefreshTokenEncrypted,
}