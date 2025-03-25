use crate::{
    channel_subscription_token_signed::ChannelSubscriptionTokenSigned,
    common_precedent::CommonPrecedent,
    user_access_token_signed::UserAccessTokenSigned,
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
    #[cfg_attr(
        feature = "serde_for_manual_test",
        serde(borrow)
    )]
    pub user_access_token_signed: UserAccessTokenSigned<'a>,
    pub channel__id: i64,
    pub channel_subscription_token_signed: ChannelSubscriptionTokenSigned,
}
crate::common_precedent::enum_from!(
    pub enum Precedent {
        CommonPrecedent::UserAccessToken__AlreadyExpired,
        CommonPrecedent::ChannelSubscriptionToken__AlreadyExpired,
        CommonPrecedent::ChannelSubscription__NotFound,
        CommonPrecedent::Channel__NotFound,
    }
);
