use entity::application_user::ApplicationUser_Id;
use entity::application_user_device::ApplicationUserDevice_Id;
use entity::application_user_reset_password_token::ApplicationUserResetPasswordToken_Value;
use serde::Deserialize;
use entity::application_user_reset_password_token::ApplicationUserResetPasswordToken_WrongEnterTriesQuantity;
use serde::Serialize;
use macro_rules::r#enum;
use common_precedent::CommonPrecedent;

#[cfg_attr(
    feature = "manual_testing",
    derive(Serialize)
)]
#[derive(Deserialize)]
pub struct Incoming {
    pub application_user_id: ApplicationUser_Id,
    pub application_user_device_id: ApplicationUserDevice_Id,
    pub application_user_reset_password_token_value: ApplicationUserResetPasswordToken_Value,
}

r#enum!(
    pub enum Precedent {
        CommonPrecedent::ApplicationUserResetPasswordToken_NotFound,
        CommonPrecedent::ApplicationUserResetPasswordToken_AlreadyExpired,
        CommonPrecedent::ApplicationUserResetPasswordToken_AlreadyApproved,
        CommonPrecedent::ApplicationUserResetPasswordToken_WrongValue {
            application_user_reset_password_token_wrong_enter_tries_quantity: ApplicationUserResetPasswordToken_WrongEnterTriesQuantity,
        },
    }
);