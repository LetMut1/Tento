use common_precedent::CommonPrecedent;
use macro_rules::r#enum;
use serde::{
    Deserialize,
    Serialize,
};
use bitcode::{
    Encode,
    Decode,
};
#[derive(Serialize, Deserialize, Encode, Decode)]
pub struct Incoming {
    pub user__email: String,
    pub user_device__id: String,
}
#[derive(Serialize, Deserialize, Encode, Decode)]
pub struct Outcoming {
    pub user__id: i64,
    pub verification_message_sent: bool,
    pub user_reset_password_token__can_be_resent_from: i64,
    pub user_reset_password_token__wrong_enter_tries_quantity: i16,
    pub user_reset_password_token__wrong_enter_tries_quantity_limit: i16,
}
r#enum!(
    pub enum Precedent {
        CommonPrecedent::User_NotFound,
    }
);
