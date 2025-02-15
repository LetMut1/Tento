use crate::{
    common_precedent::CommonPrecedent,
    user_access_token_encoded::UserAccessTokenEncoded,
};
#[cfg_attr(
    feature = "serde_for_manual_test",
    derive(
        serde::Serialize,
        serde::Deserialize
    )
)]
#[derive(bitcode::Encode, bitcode::Decode)]
pub struct Incoming {
    pub user_access_token_encoded: UserAccessTokenEncoded,
    pub channel__name: String,
    pub requery___channel__name: Option<String>,
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
    pub channel__id: i64,
    pub channel__name: String,
    pub channel__linked_name: String,
    pub channel__access_modifier: i16,
    pub channel__visability_modifier: i16,
    pub channel__cover_image_path: Option<String>,
    pub channel__background_image_path: Option<String>,
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
    }
);
