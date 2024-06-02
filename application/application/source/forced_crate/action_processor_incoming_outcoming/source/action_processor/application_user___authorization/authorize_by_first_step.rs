use serde::Deserialize;
use serde::Serialize;
use macro_rules::r#enum;
use common_precedent::CommonPrecedent;

#[derive(Serialize, Deserialize)]
pub struct Incoming {
    pub application_user_device_id: String,
    pub application_user_email_or_application_user_nickname: String,
    pub application_user_password: String,
}

#[derive(Serialize, Deserialize)]
pub struct Outcoming {
    pub application_user_id: i64,
    pub verification_message_sent: bool,
    pub application_user_authorization_token_can_be_resent_from: i64,
    pub application_user_authorization_token_wrong_enter_tries_quantity: i16,
    pub application_user_authorization_token_wrong_enter_tries_quantity_limit: i16,
}

r#enum!(
    pub enum Precedent {
        CommonPrecedent::ApplicationUser_WrongEmailOrNicknameOrPassword,
    }
);