use crate::{
    common_precedent::CommonPrecedent,
    user_access_token_signed::UserAccessTokenSigned,
    channel_subscription_token_signed::ChannelSubscriptionTokenSigned,
    channel_token_hashed::ChannelTokenHashed,
};
#[cfg_attr(
    feature = "serde_for_manual_test",
    derive(
        serde::Serialize,
        serde::Deserialize
    )
)]
#[derive(bitcode::Encode, bitcode::Decode)]
pub struct Incoming<'a> {
    #[cfg_attr(feature = "serde_for_manual_test", serde(borrow))]
    pub user_access_token_signed: UserAccessTokenSigned<'a>,
    pub channel__id: i64,
    // It CAN (not MUST) be Option::None for users already subscribed on this channel
    pub channel_token_hashed: Option<ChannelTokenHashed>,
}
#[cfg_attr(
    feature = "serde_for_manual_test",
    derive(
        serde::Serialize,
        serde::Deserialize
    )
)]
#[derive(bitcode::Encode, bitcode::Decode)]
pub struct Outcoming {
    pub channel__name: String,
    pub channel__linked_name: String,
    pub channel__description: Option<String>,
    pub channel__access_modifier: i16,
    pub channel__visability_modifier: i16,
    pub channel__orientation: Vec<i16>,
    pub channel__cover_image_path: Option<String>,
    pub channel__background_image_path: Option<String>,
    pub channel__subscribers_quantity: i64,
    pub user_is_channel_owner: bool,
    pub channel_subscription_token_signed: ChannelSubscriptionTokenSigned,
}
crate::common_precedent::enum_from!(
    pub enum Precedent {
        CommonPrecedent::UserAccessToken__AlreadyExpired,
        CommonPrecedent::Channel__NotFound,
        CommonPrecedent::Channel__IsClose,
        CommonPrecedent::ChannelToken__NotFound,
        CommonPrecedent::ChannelToken__AlreadyExpired,
    }
);