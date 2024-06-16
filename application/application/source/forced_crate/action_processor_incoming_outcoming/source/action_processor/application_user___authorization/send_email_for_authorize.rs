use common_precedent::CommonPrecedent;
use macro_rules::r#enum;
use serde::{
    Deserialize,
    Serialize,
};
#[derive(Serialize, Deserialize)]
pub struct Incoming {
    pub application_user_device__id: String,
    pub application_user__id: i64,
}
#[derive(Serialize, Deserialize)]
pub struct Outcoming {
    pub application_user_authorization_token__can_be_resent_from: i64,
}
r#enum!(
    pub enum Precedent {
        CommonPrecedent::ApplicationUser_NotFound,
        CommonPrecedent::ApplicationUserAuthorizationToken_NotFound,
        CommonPrecedent::ApplicationUserAuthorizationToken_AlreadyExpired,
        CommonPrecedent::ApplicationUserAuthorizationToken_TimeToResendHasNotCome,
    }
);
