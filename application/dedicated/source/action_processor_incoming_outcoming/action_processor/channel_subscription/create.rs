use crate::{
    common_precedent::CommonPrecedent,
    user_access_token_signed::UserAccessTokenSigned,
    channel_subscription_token_hashed::ChannelSubscriptionTokenHashed,
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
    pub channel_subscription_token_hashed: ChannelSubscriptionTokenHashed,
}
crate::common_precedent::enum_from!(
    pub enum Precedent {
        CommonPrecedent::UserAccessToken_AlreadyExpired,
        CommonPrecedent::UserAccessToken_InUserAccessTokenBlackList,
        CommonPrecedent::Channel_NotFound,
        CommonPrecedent::Channel_IsClose,
        CommonPrecedent::User_IsChannelOwner,
        CommonPrecedent::ChannelSubscription_AlreadyExist,
        CommonPrecedent::ChannelSubscriptionToken_AlreadyExpired,
    }
);
