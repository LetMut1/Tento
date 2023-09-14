use entity::application_user::ApplicationUser_Id;
use entity::application_user_authorization_token::ApplicationUserAuthorizationToken_CanBeResentFrom;
use entity::application_user_device::ApplicationUserDevice_Id;
use serde::Deserialize;
use serde::Serialize;

#[cfg_attr(
    feature = "manual_testing",
    derive(Serialize)
)]
#[derive(Deserialize)]
pub struct Incoming {
    pub application_user_device_id: ApplicationUserDevice_Id,
    pub application_user_id: ApplicationUser_Id,
}

#[cfg_attr(
    feature = "manual_testing",
    derive(Deserialize)
)]
#[derive(Serialize)]
pub struct Outcoming {
    pub application_user_authorization_token_can_be_resent_from: ApplicationUserAuthorizationToken_CanBeResentFrom,
}
