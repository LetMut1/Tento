use entity::application_user::ApplicationUser_Id;
use entity::application_user::ApplicationUser_Password;
use entity::application_user_device::ApplicationUserDevice_Id;
use entity::application_user_reset_password_token::ApplicationUserResetPasswordToken_Value;
use serde::Deserialize;
use serde::Serialize;
use macro_rules::r#enum;
use common_precedent::CommonPrecedent;

#[cfg_attr(
    feature = "manual_testing",
    derive(Serialize)
)]
#[derive(Deserialize)]
pub struct Incoming {
    pub application_user_device_id: ApplicationUserDevice_Id,
    pub application_user_id: ApplicationUser_Id,
    pub application_user_password: ApplicationUser_Password,
    pub application_user_reset_password_token_value: ApplicationUserResetPasswordToken_Value,
}

r#enum!(
    pub enum Precedent {
        CommonPrecedent::ApplicationUser_NotFound,
        CommonPrecedent::ApplicationUserResetPasswordToken_NotFound,
        CommonPrecedent::ApplicationUserResetPasswordToken_AlreadyExpired,
        CommonPrecedent::ApplicationUserResetPasswordToken_IsNotApproved,
        CommonPrecedent::ApplicationUserResetPasswordToken_WrongValue,
    }
);