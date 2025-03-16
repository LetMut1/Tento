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
    pub user__id: i64,
    pub user_device__id: &'a str,
    pub user_reset_password_token__value: &'a str,
}
crate::common_precedent::enum_from!(
    pub enum Precedent {
        CommonPrecedent::UserResetPasswordToken__NotFound,
        CommonPrecedent::UserResetPasswordToken__AlreadyExpired,
        CommonPrecedent::UserResetPasswordToken__AlreadyApproved,
        CommonPrecedent::UserResetPasswordToken__WrongValue {
            user_reset_password_token__wrong_enter_tries_quantity: i16,
        },
        CommonPrecedent::ParallelExecution,
    }
);
