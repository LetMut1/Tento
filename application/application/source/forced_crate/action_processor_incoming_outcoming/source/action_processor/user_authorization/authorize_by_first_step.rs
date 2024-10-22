use common_precedent::CommonPrecedent;
use macro_rules::enum_from;
#[cfg_attr(feature = "serde_for_manual_test", derive(serde::Serialize, serde::Deserialize))]
#[derive(bitcode::Encode, bitcode::Decode)]
pub struct Incoming {
    pub user_device__id: String,
    pub user__email___or___user__nickname: String,
    pub user__password: String,
}
#[cfg_attr(feature = "serde_for_manual_test", derive(serde::Serialize, serde::Deserialize))]
#[derive(bitcode::Encode, bitcode::Decode)]
pub struct Outcoming {
    pub user__id: i64,
    pub verification_message_sent: bool,
    pub user_authorization_token__can_be_resent_from: i64,
    pub user_authorization_token__wrong_enter_tries_quantity: i16,
    pub user_authorization_token__wrong_enter_tries_quantity_limit: i16,
}
enum_from!(
    pub enum Precedent {
        CommonPrecedent::User_WrongEmailOrNicknameOrPassword,
    }
);
