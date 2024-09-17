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
    pub channel__id: i64,
}
#[derive(Serialize, Deserialize)]
pub struct Outcoming {
    pub channel: Channel2,
    pub channel_inner_link_registry: Vec<ChannelInnerLink1>,
    pub channel_outer_link_registry: Vec<ChannelOuterLink1>,
}
r#enum!(
    pub enum Precedent {
        CommonPrecedent::ApplicationUserAccessToken_AlreadyExpired,
        CommonPrecedent::ApplicationUserAccessToken_InApplicationUserAccessTokenBlackList,
        CommonPrecedent::Channel_NotFound,
        CommonPrecedent::Channel_IsClose,
    }
);
