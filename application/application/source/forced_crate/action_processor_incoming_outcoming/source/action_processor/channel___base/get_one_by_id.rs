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

#[cfg_attr(
    feature = "manual_testing",
    derive(Serialize)
)]
#[derive(Deserialize)]
pub struct Incoming {
    application_user_access_token_encrypted: ApplicationUserAccessTokenEncrypted,
    channel_id: Channel_Id,
}

#[cfg_attr(
    feature = "manual_testing",
    derive(Deserialize)
)]
#[derive(Serialize)]
pub struct Outcoming {
    channel: Channel,
    channel_inner_link_registry: Vec<ChannelInnerLink1>,
    channel_outer_link_registry: Vec<ChannelOuterLink1>,
}

#[cfg_attr(
    feature = "manual_testing",
    derive(Deserialize)
)]
#[derive(Serialize)]
struct Channel {
    channel_owner: ApplicationUser_Id,
    channel_name: Channel_Name,
    channel_linked_name: Channel_LinkedName,
    channel_description: Option<Channel_Description>,
    channel_access_modifier: Channel_AccessModifier,
    channel_visability_modifier: Channel_VisabilityModifier,
    channel_orientation: Channel_Orientation,
    channel_cover_image_path: Option<Channel_CoverImagePath>,
    channel_background_image_path: Option<Channel_BackgroundImagePath>,
    channel_subscribers_quantity: Channel_SubscribersQuantity,
    channel_marks_quantity: Channel_MarksQuantity,
    channel_viewing_quantity: Channel_ViewingQuantity,
}
