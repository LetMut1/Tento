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
    pub user_device__id: String,
    pub user__id: i64,
}
#[derive(Serialize, Deserialize, Encode, Decode)]
pub struct Outcoming {
    pub user_authorization_token__can_be_resent_from: i64,
}
r#enum!(
    pub enum Precedent {
        CommonPrecedent::User_NotFound,
        CommonPrecedent::UserAuthorizationToken_NotFound,
        CommonPrecedent::UserAuthorizationToken_AlreadyExpired,
        CommonPrecedent::UserAuthorizationToken_TimeToResendHasNotCome,
    }
);
