use crate::common_precedent::CommonPrecedent;
#[cfg_attr(
    feature = "serde_for_manual_test",
    derive(
        serde::Serialize,
        serde::Deserialize
    )
)]
#[derive(bitcode::Encode, bitcode::Decode)]
pub struct Incoming {
    pub user_device__id: String,
    pub user__id: i64,
    pub user__password: String,
    pub user_reset_password_token__value: String,
}
crate::common_precedent::enum_from!(
    pub enum Precedent {
        CommonPrecedent::User_NotFound,
        CommonPrecedent::UserResetPasswordToken_NotFound,
        CommonPrecedent::UserResetPasswordToken_AlreadyExpired,
        CommonPrecedent::UserResetPasswordToken_IsNotApproved,
        CommonPrecedent::UserResetPasswordToken_WrongValue,
    }
);
