use crate::{
    channel_publication1_token_signed::ChannelPublication1TokenSigned,
    channel_token_signed::ChannelTokenSigned,
    common_precedent::CommonPrecedent,
    user_access_token_signed::UserAccessTokenSigned,
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
    // The bitcode(=0.6.3)::Decode not implemented for &'_ [&'_ str],
    pub channel_publication1__images_pathes: Vec<&'a str>,
    // The bitcode(=0.6.3)::Decode not implemented for &'_ Option<&'_ str>.
    pub channel_publication1__text: Option<&'a str>,
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
    pub channel_publication1__created_at: i64,
    pub channel_publication1_token_signed: ChannelPublication1TokenSigned,
}
crate::common_precedent::enum_from!(
    pub enum Precedent {
        CommonPrecedent::UserAccessToken__AlreadyExpired,
        CommonPrecedent::ChannelToken__AlreadyExpired,
        CommonPrecedent::ParallelExecution,
    }
);
