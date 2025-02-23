use crate::common_precedent::CommonPrecedent;
#[cfg_attr(
    feature = "serde_for_manual_test",
    derive(
        serde::Serialize,
        serde::Deserialize
    )
)]
#[derive(bitcode::Encode, bitcode::Decode)]
pub struct Incoming<'a> {
    pub user_device__id: &'a str,
    pub user__id: i64,
    pub user__password: &'a str,
    pub user_reset_password_token__value: &'a str,
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
