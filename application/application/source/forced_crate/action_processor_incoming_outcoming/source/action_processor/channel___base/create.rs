use crate::{
    Channel2,
    ChannelInnerLink1,
    ChannelOuterLink1,
};
use application_user_access_token_encoded::ApplicationUserAccessTokenEncoded;
use common_precedent::CommonPrecedent;
use macro_rules::r#enum;
use serde::{
    Deserialize,
    Serialize,
};
#[derive(Serialize, Deserialize)]
pub struct Incoming {
    pub application_user_access_token_encoded: ApplicationUserAccessTokenEncoded,




}
#[derive(Serialize, Deserialize)]
pub struct Outcoming {
}
r#enum!(
    pub enum Precedent {
        CommonPrecedent::ApplicationUserAccessToken_AlreadyExpired,
        CommonPrecedent::ApplicationUserAccessToken_InApplicationUserAccessTokenBlackList,





    }
);