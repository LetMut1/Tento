use crate::{
    common_precedent::CommonPrecedent,
    user_access_token_signed::UserAccessTokenSigned,
    channel_subscription_token_signed::ChannelSubscriptionTokenSigned,
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
    pub channel_subscription_token_signed: ChannelSubscriptionTokenSigned,
}
crate::common_precedent::enum_from!(
    pub enum Precedent {
        CommonPrecedent::UserAccessToken__AlreadyExpired,
        CommonPrecedent::Channel__NotFound,
        CommonPrecedent::Channel__IsClose,
        CommonPrecedent::User__IsChannelOwner,
        CommonPrecedent::ChannelSubscription__AlreadyExist,
        CommonPrecedent::ChannelSubscriptionToken__AlreadyExpired,
    }
);
