use common_precedent::CommonPrecedent;
use macro_rules::r#enum;
use serde::{
    Deserialize,
    Serialize,
};
#[derive(Serialize, Deserialize)]
pub struct Incoming {
    pub application_user_device__id: String,
    pub application_user__email___or___application_user__nickname: String,
    pub application_user__password: String,
}
#[derive(Serialize, Deserialize)]
pub struct Outcoming {
    pub application_user__id: i64,
    pub verification_message_sent: bool,
    pub application_user_authorization_token__can_be_resent_from: i64,
    pub application_user_authorization_token__wrong_enter_tries_quantity: i16,
    pub application_user_authorization_token__wrong_enter_tries_quantity_limit: i16,
}
r#enum!(
    pub enum Precedent {
        CommonPrecedent::User_WrongEmailOrNicknameOrPassword,
    }
);
