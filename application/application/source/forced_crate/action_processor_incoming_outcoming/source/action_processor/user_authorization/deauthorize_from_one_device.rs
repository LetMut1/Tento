use user_access_token_encoded::UserAccessTokenEncoded;
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
    pub user_access_token_encoded: UserAccessTokenEncoded,
}
r#enum!(
    pub enum Precedent {
        CommonPrecedent::UserAccessToken_AlreadyExpired,
        CommonPrecedent::UserAccessToken_InUserAccessTokenBlackList,
    }
);
