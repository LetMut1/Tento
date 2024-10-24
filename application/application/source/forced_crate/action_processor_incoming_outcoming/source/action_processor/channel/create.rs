use user_access_token_encoded::UserAccessTokenEncoded;
use common_precedent::CommonPrecedent;
#[cfg_attr(feature = "serde_for_manual_test", derive(serde::Serialize, serde::Deserialize))]
#[derive(bitcode::Encode, bitcode::Decode)]
pub struct Incoming {
    pub user_access_token_encoded: UserAccessTokenEncoded,
    pub channel__name: String,
    pub channel__linked_name: String,
    pub channel__access_modifier: i16,
    pub channel__visability_modifier: i16,
}
#[cfg_attr(feature = "serde_for_manual_test", derive(serde::Serialize, serde::Deserialize))]
#[derive(bitcode::Encode, bitcode::Decode)]
pub struct Outcoming {
    pub channel__id: i64,
}
macro_rules::enum_from!(
    pub enum Precedent {
        CommonPrecedent::UserAccessToken_AlreadyExpired,
        CommonPrecedent::UserAccessToken_InUserAccessTokenBlackList,
        CommonPrecedent::Channel_NameAlreadyExist,
        CommonPrecedent::Channel_LinkedNameAlreadyExist
    }
);
