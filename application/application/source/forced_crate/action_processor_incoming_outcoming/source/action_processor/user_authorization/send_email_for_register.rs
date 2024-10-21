use common_precedent::CommonPrecedent;
use macro_rules::r#enum;
use serde::{
    Deserialize,
    Serialize,
};
use bitcode::{
    Encode,
    Decode,
};
#[derive(Serialize, Deserialize, Encode, Decode)]
pub struct Incoming {
    pub user__email: String,
    pub user_device__id: String,
}
#[derive(Serialize, Deserialize, Encode, Decode)]
pub struct Outcoming {
    pub user_registration_token__can_be_resent_from: i64,
}
r#enum!(
    pub enum Precedent {
        CommonPrecedent::UserRegistrationToken_NotFound,
        CommonPrecedent::UserRegistrationToken_AlreadyExpired,
        CommonPrecedent::UserRegistrationToken_AlreadyApproved,
        CommonPrecedent::UserRegistrationToken_TimeToResendHasNotCome,
    }
);
