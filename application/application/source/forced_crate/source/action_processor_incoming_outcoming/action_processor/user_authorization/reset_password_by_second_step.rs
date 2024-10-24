use crate::common_precedent::CommonPrecedent;
#[cfg_attr(feature = "serde_for_manual_test", derive(serde::Serialize, serde::Deserialize))]
#[derive(bitcode::Encode, bitcode::Decode)]
pub struct Incoming {
    pub user__id: i64,
    pub user_device__id: String,
    pub user_reset_password_token__value: String,
}
crate::macro_rules::enum_from!(
    pub enum Precedent {
        CommonPrecedent::UserResetPasswordToken_NotFound,
        CommonPrecedent::UserResetPasswordToken_AlreadyExpired,
        CommonPrecedent::UserResetPasswordToken_AlreadyApproved,
        CommonPrecedent::UserResetPasswordToken_WrongValue {
            user_reset_password_token__wrong_enter_tries_quantity: i16,
        },
    }
);
