use common_precedent::CommonPrecedent;
use macro_rules::r#enum;
use serde::{
    Deserialize,
    Serialize,
};
use bitcode::{
    Encode,
    Decode,
};
#[derive(Serialize, Deserialize, Encode, Decode)]
pub struct Incoming {
    pub user_device__id: String,
    pub user__id: i64,
    pub user__password: String,
    pub user_reset_password_token__value: String,
}
r#enum!(
    pub enum Precedent {
        CommonPrecedent::User_NotFound,
        CommonPrecedent::UserResetPasswordToken_NotFound,
        CommonPrecedent::UserResetPasswordToken_AlreadyExpired,
        CommonPrecedent::UserResetPasswordToken_IsNotApproved,
        CommonPrecedent::UserResetPasswordToken_WrongValue,
    }
);
