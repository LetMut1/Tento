use common_precedent::CommonPrecedent;
use macro_rules::enum_from;
#[cfg_attr(feature = "serde_for_manual_test", derive(serde::Serialize, serde::Deserialize))]
#[derive(bitcode::Encode, bitcode::Decode)]
pub struct Incoming {
    pub user__email: String,
    pub user_device__id: String,
    pub user_registration_token__value: String,
}
enum_from!(
    pub enum Precedent {
        CommonPrecedent::UserRegistrationToken_NotFound,
        CommonPrecedent::UserRegistrationToken_AlreadyExpired,
        CommonPrecedent::UserRegistrationToken_AlreadyApproved,
        CommonPrecedent::UserRegistrationToken_WrongValue {
            user_registration_token__wrong_enter_tries_quantity: i16,
        },
    }
);
