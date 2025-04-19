use crate::{
    channel_publication1_token_signed::ChannelPublication1TokenSigned,
    common_precedent::CommonPrecedent,
    user_access_token_signed::UserAccessTokenSigned,
    channel_token_signed::ChannelTokenSigned,
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
    pub channel_publication1__created_at: i64,
    pub limit: u8,
}
#[cfg_attr(
    feature = "serde_for_manual_test",
    derive(
        serde::Serialize,
        serde::Deserialize,
    )
)]
#[derive(bitcode::Encode, bitcode::Decode)]
pub struct Data {
    pub channel_publication1__images_pathes: Vec<String>,
    pub channel_publication1__text: Option<String>,
    pub channel_publication1__commentaries_quantity: u32,
    pub channel_publication1__marks_quantity: u32,
    pub channel_publication1__view_quantity: u32,
    pub channel_publication1__created_at: i64,
    pub channel_publication1_mark__created_at: Option<i64>,
    pub channel_publication1_token_signed: ChannelPublication1TokenSigned,
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
    pub data_registry: Vec<Data>,
}
crate::common_precedent::enum_from!(
    pub enum Precedent {
        CommonPrecedent::UserAccessToken__AlreadyExpired,
        CommonPrecedent::ChannelToken__AlreadyExpired,
        CommonPrecedent::ChannelToken__InvalidChannelOwnerDefinition,
        CommonPrecedent::Channel__NotFound,
        CommonPrecedent::Channel__IsClose,
    }
);
