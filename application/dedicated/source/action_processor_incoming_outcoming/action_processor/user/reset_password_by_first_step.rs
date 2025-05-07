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
    pub user__email: &'a str,
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
    pub user__obfuscated_id: i64,
    pub verification_message_sent: bool,
    pub user_reset_password_token__can_be_resent_from: i64,
    pub user_reset_password_token__wrong_enter_tries_quantity: u8,
    pub user_reset_password_token__wrong_enter_tries_quantity_limit: u8,
}
crate::common_precedent::enum_from!(
    pub enum Precedent {
        CommonPrecedent::User__NotFound,
        CommonPrecedent::ParallelExecution,
    }
);
