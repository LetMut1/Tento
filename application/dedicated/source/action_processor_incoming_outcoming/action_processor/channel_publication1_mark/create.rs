use crate::{
    common_precedent::CommonPrecedent,
    user_access_token_signed::UserAccessTokenSigned,
    channel_publication1_token_signed::ChannelPublication1TokenSigned,
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
    pub channel_publication1__id: i64,
    pub channel_publication1_token_signed: ChannelPublication1TokenSigned,
}
crate::common_precedent::enum_from!(
    pub enum Precedent {
        CommonPrecedent::UserAccessToken__AlreadyExpired,
        CommonPrecedent::ChannelPublication1Token__AlreadyExist,
        CommonPrecedent::ChannelPublication1Mark__AlreadyExist,
        CommonPrecedent::ChannelPublication1__NotFound,
    }
);
