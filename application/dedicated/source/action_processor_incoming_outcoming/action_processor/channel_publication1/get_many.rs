use crate::{
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
    #[cfg_attr(feature = "serde_for_manual_test", serde(borrow))]
    pub user_access_token_signed: UserAccessTokenSigned<'a>,
    pub channel__id: i64,
    pub channel_publication1__created_at: i64,
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
    pub channel_publication1__id: i64,
    pub channel_publication1__images_pathes: Vec<String>,
    pub channel_publication1__text: Option<String>,
    pub channel_publication1__marks_quantity: i64,
    pub channel_publication1__viewing_quantity: i64,
    pub channel_publication1__created_at: i64,
    pub channel_publication1_mark__created_at: Option<i64>,
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
        CommonPrecedent::UserAccessToken_AlreadyExpired,
        CommonPrecedent::UserAccessToken_InUserAccessTokenBlackList,
        CommonPrecedent::Channel_NotFound,
        CommonPrecedent::Channel_IsClose,
    }
);
