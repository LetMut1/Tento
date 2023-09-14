use entity::application_user::ApplicationUser_Email;
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
    pub application_user_email: ApplicationUser_Email,
    pub application_user_device_id: ApplicationUserDevice_Id,
    pub application_user_registration_token_value: ApplicationUserRegistrationToken_Value,
}