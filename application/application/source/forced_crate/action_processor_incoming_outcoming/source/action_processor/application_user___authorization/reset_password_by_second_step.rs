use common_precedent::CommonPrecedent;
use macro_rules::r#enum;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize)]
pub struct Incoming {
    pub application_user_id: i64,
    pub application_user_device_id: String,
    pub application_user_reset_password_token_value: String,
}

r#enum!(
    pub enum Precedent {
        CommonPrecedent::ApplicationUserResetPasswordToken_NotFound,
        CommonPrecedent::ApplicationUserResetPasswordToken_AlreadyExpired,
        CommonPrecedent::ApplicationUserResetPasswordToken_AlreadyApproved,
        CommonPrecedent::ApplicationUserResetPasswordToken_WrongValue {
            application_user_reset_password_token_wrong_enter_tries_quantity: i16,
        },
    }
);
