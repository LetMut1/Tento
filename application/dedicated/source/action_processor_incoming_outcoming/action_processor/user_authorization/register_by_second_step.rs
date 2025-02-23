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
        CommonPrecedent::UserRegistrationToken_NotFound,
        CommonPrecedent::UserRegistrationToken_AlreadyExpired,
        CommonPrecedent::UserRegistrationToken_AlreadyApproved,
        CommonPrecedent::UserRegistrationToken_WrongValue {
            user_registration_token__wrong_enter_tries_quantity: i16,
        },
    }
);
