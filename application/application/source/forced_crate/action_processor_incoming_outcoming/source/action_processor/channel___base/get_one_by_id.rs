use common_precedent::CommonPrecedent;
use crate::ChannelInnerLink1;
use crate::ChannelOuterLink1;
use entity::channel::Channel_Id;
use macro_rules::r#enum;
use serde::Deserialize;
use serde::Serialize;

pub use crate::Channel2;

#[derive(Serialize, Deserialize)]
pub struct Incoming {
    pub application_user_access_token_encrypted: String,
    pub channel_id: Channel_Id,
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