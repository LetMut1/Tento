use crate::{
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
    pub user_access_token_signed: UserAccessTokenSigned<'a>,
    pub channel__name: &'a str,
    pub channel__linked_name: &'a str,
    pub channel__access_modifier: i16,
    pub channel__visability_modifier: i16,
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
    pub channel_token_signed: ChannelTokenSigned,
}
crate::common_precedent::enum_from!(
    pub enum Precedent {
        CommonPrecedent::UserAccessToken__AlreadyExpired,
        CommonPrecedent::Channel__NameAlreadyExist,
        CommonPrecedent::Channel__LinkedNameAlreadyExist,
        CommonPrecedent::ParallelExecution,
    }
);
