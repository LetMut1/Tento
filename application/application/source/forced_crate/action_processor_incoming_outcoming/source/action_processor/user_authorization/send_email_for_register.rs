use common_precedent::CommonPrecedent;
use macro_rules::r#enum;
use serde::{
    Deserialize,
    Serialize,
};
#[derive(Serialize, Deserialize)]
pub struct Incoming {
    pub application_user__email: String,
    pub application_user_device__id: String,
}
#[derive(Serialize, Deserialize)]
pub struct Outcoming {
    pub application_user_registration_token__can_be_resent_from: i64,
}
r#enum!(
    pub enum Precedent {
        CommonPrecedent::UserRegistrationToken_NotFound,
        CommonPrecedent::UserRegistrationToken_AlreadyExpired,
        CommonPrecedent::UserRegistrationToken_AlreadyApproved,
        CommonPrecedent::UserRegistrationToken_TimeToResendHasNotCome,
    }
);