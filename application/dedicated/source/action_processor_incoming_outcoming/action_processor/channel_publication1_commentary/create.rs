use crate::{
    channel_publication1_commentary_token_signed::ChannelPublication1CommentaryTokenSigned, channel_publication1_token_signed::ChannelPublication1TokenSigned, channel_token_signed::ChannelTokenSigned, common_precedent::CommonPrecedent, user_access_token_signed::UserAccessTokenSigned
};
#[cfg_attr(
    feature = "serde_for_manual_test",
    derive(
        serde::Serialize,
        serde::Deserialize,
    )
)]
#[derive(bitcode::Encode, bitcode::Decode)]
pub struct Incoming<'a> {
    #[cfg_attr(
        feature = "serde_for_manual_test",
        serde(borrow)
    )]
    pub user_access_token_signed: UserAccessTokenSigned<'a>,
    pub channel_token_signed: ChannelTokenSigned,
    pub channel_publication1_token_signed: ChannelPublication1TokenSigned,
    pub channel_publication1_commentary__text: &'a str,
}
#[cfg_attr(
    feature = "serde_for_manual_test",
    derive(
        serde::Serialize,
        serde::Deserialize,
    )
)]
#[derive(bitcode::Encode, bitcode::Decode)]
pub struct Outcoming {
    pub channel_publication1_commentary__created_at: i64,
    pub channel_publication1_commentary_token_signed: ChannelPublication1CommentaryTokenSigned,
}
crate::common_precedent::enum_from!(
    pub enum Precedent {
        CommonPrecedent::UserAccessToken__AlreadyExpired,
        CommonPrecedent::ChannelToken__AlreadyExpired,
        CommonPrecedent::ChannelPublication1Token__AlreadyExpired,
        CommonPrecedent::ParallelExecution,
    }
);
