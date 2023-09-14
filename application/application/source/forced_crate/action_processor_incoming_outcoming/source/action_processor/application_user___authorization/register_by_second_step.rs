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
    application_user_email: ApplicationUser_Email,
    application_user_device_id: ApplicationUserDevice_Id,
    application_user_registration_token_value: ApplicationUserRegistrationToken_Value,
}