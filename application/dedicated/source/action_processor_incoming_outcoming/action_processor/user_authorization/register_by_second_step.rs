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
    pub user__email: &'a str,
    pub user_device__id: &'a str,
    pub user_registration_token__value: &'a str,
}
crate::common_precedent::enum_from!(
    pub enum Precedent {
        CommonPrecedent::UserRegistrationToken__NotFound,
        CommonPrecedent::UserRegistrationToken__AlreadyExpired,
        CommonPrecedent::UserRegistrationToken__AlreadyApproved,
        CommonPrecedent::UserRegistrationToken__WrongValue {
            user_registration_token__wrong_enter_tries_quantity: i16,
        },
        CommonPrecedent::ParallelExecution,
    }
);
