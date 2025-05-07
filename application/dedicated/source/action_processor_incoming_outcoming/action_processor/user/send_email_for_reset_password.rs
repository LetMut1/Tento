use crate::common_precedent::CommonPrecedent;
#[cfg_attr(
    feature = "serde_for_manual_test",
    derive(
        serde::Serialize,
        serde::Deserialize,
    )
)]
#[derive(bitcode::Encode, bitcode::Decode)]
pub struct Incoming<'a> {
    pub user__obfuscated_id: i64,
    pub user_device__id: &'a str,
}
#[cfg_attr(
    feature = "serde_for_manual_test",
    derive(
        serde::Serialize,
        serde::Deserialize,
    )
)]
#[derive(bitcode::Encode, bitcode::Decode)]
pub struct Outcoming {
    pub user_reset_password_token__can_be_resent_from: i64,
}
crate::common_precedent::enum_from!(
    pub enum Precedent {
        CommonPrecedent::User__NotFound,
        CommonPrecedent::UserResetPasswordToken__NotFound,
        CommonPrecedent::UserResetPasswordToken__AlreadyExpired,
        CommonPrecedent::UserResetPasswordToken__AlreadyApproved,
        CommonPrecedent::UserResetPasswordToken__TimeToResendHasNotCome,
        CommonPrecedent::ParallelExecution,
    }
);
