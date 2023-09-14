use entity::application_user::ApplicationUser_Email;
use entity::application_user::ApplicationUser_Id;
use entity::application_user_device::ApplicationUserDevice_Id;
use entity::application_user_reset_password_token::ApplicationUserResetPasswordToken_CanBeResentFrom;
use entity::application_user_reset_password_token::ApplicationUserResetPasswordToken_WrongEnterTriesQuantity;
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
    application_user_id: ApplicationUser_Id,
    verification_message_sent: bool,
    application_user_reset_password_token_can_be_resent_from: ApplicationUserResetPasswordToken_CanBeResentFrom,
    application_user_reset_password_token_wrong_enter_tries_quantity: ApplicationUserResetPasswordToken_WrongEnterTriesQuantity,
    application_user_reset_password_token_wrong_enter_tries_quantity_limit: i16,
}
