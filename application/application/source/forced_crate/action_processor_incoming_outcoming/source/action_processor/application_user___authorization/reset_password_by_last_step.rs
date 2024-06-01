use entity::application_user_reset_password_token::ApplicationUserResetPasswordToken_Value;
use serde::Deserialize;
use serde::Serialize;
use macro_rules::r#enum;
use common_precedent::CommonPrecedent;

#[derive(Serialize, Deserialize)]
pub struct Incoming {
    pub application_user_device_id: String,
    pub application_user_id: i64,
    pub application_user_password: String,
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