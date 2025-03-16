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
    pub user__email___or___user__nickname: &'a str,
    pub user__password: &'a str,
}
#[cfg_attr(
    feature = "serde_for_manual_test",
    derive(
        serde::Serialize,
        serde::Deserialize
    )
)]
#[derive(bitcode::Encode, bitcode::Decode)]
pub struct Outcoming {
    pub user__id: i64,
    pub verification_message_sent: bool,
    pub user_authorization_token__can_be_resent_from: i64,
    pub user_authorization_token__wrong_enter_tries_quantity: i16,
    pub user_authorization_token__wrong_enter_tries_quantity_limit: i16,
}
crate::common_precedent::enum_from!(
    pub enum Precedent {
        CommonPrecedent::User__WrongEmailOrNicknameOrPassword,
        CommonPrecedent::DeletedInParallelExecution,
    }
);
