use crate::ChannelInnerLink1;
use crate::ChannelOuterLink1;
use entity::application_user_access_token_encrypted::ApplicationUserAccessTokenEncrypted;
use entity::application_user::ApplicationUser_Id;
use entity::channel::Channel_AccessModifier;
use entity::channel::Channel_BackgroundImagePath;
use entity::channel::Channel_CoverImagePath;
use entity::channel::Channel_Description;
use entity::channel::Channel_Id;
use entity::channel::Channel_LinkedName;
use entity::channel::Channel_MarksQuantity;
use entity::channel::Channel_Name;
use entity::channel::Channel_Orientation;
use entity::channel::Channel_SubscribersQuantity;
use entity::channel::Channel_ViewingQuantity;
use entity::channel::Channel_VisabilityModifier;
use serde::Deserialize;
use serde::Serialize;
use macro_rules::r#enum;
use common_precedent::CommonPrecedent;

#[cfg_attr(
    feature = "manual_testing",
    derive(Serialize)
)]
#[derive(Deserialize)]
pub struct Incoming {
    pub application_user_access_token_encrypted: ApplicationUserAccessTokenEncrypted,
    pub channel_id: Channel_Id,
}

#[cfg_attr(
    feature = "manual_testing",
    derive(Deserialize)
)]
#[derive(Serialize)]
pub struct Outcoming {
    pub channel: Channel,
    pub channel_inner_link_registry: Vec<ChannelInnerLink1>,
    pub channel_outer_link_registry: Vec<ChannelOuterLink1>,
}

#[cfg_attr(
    feature = "manual_testing",
    derive(Deserialize)
)]
#[derive(Serialize)]
pub struct Channel {
    pub channel_owner: ApplicationUser_Id,
    pub channel_name: Channel_Name,
    pub channel_linked_name: Channel_LinkedName,
    pub channel_description: Option<Channel_Description>,
    pub channel_access_modifier: Channel_AccessModifier,
    pub channel_visability_modifier: Channel_VisabilityModifier,
    pub channel_orientation: Channel_Orientation,
    pub channel_cover_image_path: Option<Channel_CoverImagePath>,
    pub channel_background_image_path: Option<Channel_BackgroundImagePath>,
    pub channel_subscribers_quantity: Channel_SubscribersQuantity,
    pub channel_marks_quantity: Channel_MarksQuantity,
    pub channel_viewing_quantity: Channel_ViewingQuantity,
}

r#enum!(
    pub enum Precedent {
        CommonPrecedent::ApplicationUserAccessToken_AlreadyExpired,
        CommonPrecedent::ApplicationUserAccessToken_InApplicationUserAccessTokenBlackList,
        CommonPrecedent::Channel_NotFound,
        CommonPrecedent::Channel_IsClosed,
    }
);