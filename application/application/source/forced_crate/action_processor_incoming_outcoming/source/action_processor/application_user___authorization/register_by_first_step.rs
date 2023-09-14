use entity::application_user::ApplicationUser_Email;
use entity::application_user_device::ApplicationUserDevice_Id;
use entity::application_user_registration_token::ApplicationUserRegistrationToken_CanBeResentFrom;
use entity::application_user_registration_token::ApplicationUserRegistrationToken_WrongEnterTriesQuantity;
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
}

#[cfg_attr(
    feature = "manual_testing",
    derive(Deserialize)
)]
#[derive(Serialize)]
pub struct Outcoming {
    verification_message_sent: bool,
    application_user_registration_token_can_be_resent_from: ApplicationUserRegistrationToken_CanBeResentFrom,
    application_user_registration_token_wrong_enter_tries_quantity: ApplicationUserRegistrationToken_WrongEnterTriesQuantity,
    application_user_registration_token_wrong_enter_tries_quantity_limit: i16,
}