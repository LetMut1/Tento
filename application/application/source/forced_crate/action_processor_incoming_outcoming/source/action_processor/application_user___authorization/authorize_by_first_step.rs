use entity::application_user::ApplicationUser_Id;
use entity::application_user::ApplicationUser_Password;
use entity::application_user_authorization_token::ApplicationUserAuthorizationToken_CanBeResentFrom;
use entity::application_user_authorization_token::ApplicationUserAuthorizationToken_WrongEnterTriesQuantity;
use entity::application_user_device::ApplicationUserDevice_Id;
use serde::Deserialize;
use serde::Serialize;

#[cfg_attr(
    feature = "manual_testing",
    derive(Serialize)
)]
#[derive(Deserialize)]
pub struct Incoming {
    application_user_device_id: ApplicationUserDevice_Id,
    application_user_email_or_application_user_nickname: String,
    application_user_password: ApplicationUser_Password,
}

#[cfg_attr(
    feature = "manual_testing",
    derive(Deserialize)
)]
#[derive(Serialize)]
pub struct Outcoming {
    application_user_id: ApplicationUser_Id,
    verification_message_sent: bool,
    application_user_authorization_token_can_be_resent_from: ApplicationUserAuthorizationToken_CanBeResentFrom,
    application_user_authorization_token_wrong_enter_tries_quantity: ApplicationUserAuthorizationToken_WrongEnterTriesQuantity,
    application_user_authorization_token_wrong_enter_tries_quantity_limit: i16,
}