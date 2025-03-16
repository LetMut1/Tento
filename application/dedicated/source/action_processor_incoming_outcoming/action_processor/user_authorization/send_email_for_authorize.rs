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
    pub user_authorization_token__can_be_resent_from: i64,
}
crate::common_precedent::enum_from!(
    pub enum Precedent {
        CommonPrecedent::User__NotFound,
        CommonPrecedent::UserAuthorizationToken__NotFound,
        CommonPrecedent::UserAuthorizationToken__AlreadyExpired,
        CommonPrecedent::UserAuthorizationToken__TimeToResendHasNotCome,
        CommonPrecedent::ParallelExecution,
    }
);
