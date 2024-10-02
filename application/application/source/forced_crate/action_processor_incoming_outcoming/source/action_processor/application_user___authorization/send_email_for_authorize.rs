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
        CommonPrecedent::User_NotFound,
        CommonPrecedent::UserAuthorizationToken_NotFound,
        CommonPrecedent::UserAuthorizationToken_AlreadyExpired,
        CommonPrecedent::UserAuthorizationToken_TimeToResendHasNotCome,
    }
);
