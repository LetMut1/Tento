use entity::application_user::ApplicationUser_Id;
use entity::application_user_device::ApplicationUserDevice_Id;
use entity::application_user_reset_password_token::ApplicationUserResetPasswordToken_CanBeResentFrom;
use serde::Deserialize;
use serde::Serialize;

#[cfg_attr(
    feature = "manual_testing",
    derive(Serialize)
)]
#[derive(Deserialize)]
pub struct Incoming {
    application_user_id: ApplicationUser_Id,
    application_user_device_id: ApplicationUserDevice_Id,
}

#[cfg_attr(
    feature = "manual_testing",
    derive(Deserialize)
)]
#[derive(Serialize)]
pub struct Outcoming {
    application_user_registration_token_can_be_resent_from: ApplicationUserResetPasswordToken_CanBeResentFrom,
}