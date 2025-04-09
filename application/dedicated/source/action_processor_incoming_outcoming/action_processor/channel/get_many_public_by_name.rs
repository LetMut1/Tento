use crate::{
    channel_token_signed::ChannelTokenSigned,
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
    pub user_access_token_signed: UserAccessTokenSigned<'a>,
    pub channel__name: &'a str,
    // The bitcode(=0.6.3)::Decode not implemented for &'_ Option<&'_ str>.
    pub requery___channel__name: Option<&'a str>,
    pub limit: i16,
}
#[cfg_attr(
    feature = "serde_for_manual_test",
    derive(
        serde::Serialize,
        serde::Deserialize
    )
)]
#[derive(bitcode::Encode, bitcode::Decode)]
pub struct Data {
    pub channel__name: String,
    pub channel__linked_name: String,
    pub channel__access_modifier: i16,
    pub channel__cover_image_path: Option<String>,
    pub channel__background_image_path: Option<String>,
    pub channel_token_signed: ChannelTokenSigned,
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
    pub data_registry: Vec<Data>,
}
crate::common_precedent::enum_from!(
    pub enum Precedent {
        CommonPrecedent::UserAccessToken__AlreadyExpired,
    }
);
