use common_precedent::CommonPrecedent;
use macro_rules::r#enum;
use serde::{
    Deserialize,
    Serialize,
};
#[derive(Serialize, Deserialize)]
pub struct Incoming {
    pub application_user__id: i64,
    pub application_user_device__id: String,
    pub application_user_reset_password_token__value: String,
}
r#enum!(
    pub enum Precedent {
        CommonPrecedent::UserResetPasswordToken_NotFound,
        CommonPrecedent::UserResetPasswordToken_AlreadyExpired,
        CommonPrecedent::UserResetPasswordToken_AlreadyApproved,
        CommonPrecedent::UserResetPasswordToken_WrongValue {
            application_user_reset_password_token__wrong_enter_tries_quantity: i16,
        },
    }
);
