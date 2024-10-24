use crate::{
    Channel2,
    ChannelInnerLink1,
    ChannelOuterLink1,
};
use user_access_token_encoded::UserAccessTokenEncoded;
use common_precedent::CommonPrecedent;
#[cfg_attr(feature = "serde_for_manual_test", derive(serde::Serialize, serde::Deserialize))]
#[derive(bitcode::Encode, bitcode::Decode)]
pub struct Incoming {
    pub user_access_token_encoded: UserAccessTokenEncoded,
    pub channel__id: i64,
}
#[cfg_attr(feature = "serde_for_manual_test", derive(serde::Serialize, serde::Deserialize))]
#[derive(bitcode::Encode, bitcode::Decode)]
pub struct Outcoming {
    pub channel: Channel2,
    pub channel_inner_link_registry: Vec<ChannelInnerLink1>,
    pub channel_outer_link_registry: Vec<ChannelOuterLink1>,
}
macro_rules::enum_from!(
    pub enum Precedent {
        CommonPrecedent::UserAccessToken_AlreadyExpired,
        CommonPrecedent::UserAccessToken_InUserAccessTokenBlackList,
        CommonPrecedent::Channel_NotFound,
        CommonPrecedent::Channel_IsClose,
    }
);
