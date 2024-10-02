use application_user_access_token_encoded::UserAccessTokenEncoded;
use common_precedent::CommonPrecedent;
use macro_rules::r#enum;
use serde::{
    Deserialize,
    Serialize,
};
#[derive(Serialize, Deserialize)]
pub struct Incoming {
    pub application_user_access_token_encoded: UserAccessTokenEncoded,
    pub channel__id: i64,
}
r#enum!(
    pub enum Precedent {
        CommonPrecedent::UserAccessToken_AlreadyExpired,
        CommonPrecedent::UserAccessToken_InUserAccessTokenBlackList,
        CommonPrecedent::Channel_NotFound,
        CommonPrecedent::Channel_IsClose,
        CommonPrecedent::User_IsChannelOwner,
    }
);
