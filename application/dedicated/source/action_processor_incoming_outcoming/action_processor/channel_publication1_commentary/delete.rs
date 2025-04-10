use crate::{
    channel_publication1_token_signed::ChannelPublication1TokenSigned,
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
    pub channel_publication1_commentary__id: i64,
    pub channel_publication1_token_signed: ChannelPublication1TokenSigned,
}
crate::common_precedent::enum_from!(
    pub enum Precedent {
        CommonPrecedent::UserAccessToken__AlreadyExpired,
        CommonPrecedent::ChannelPublication1Token__AlreadyExpired,
        CommonPrecedent::ChannelPublication1Commentary__NotFound,
    }
);
