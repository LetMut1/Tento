use entity::application_user_authorization_token::ApplicationUserAuthorizationToken_CanBeResentFrom;
use entity::application_user_authorization_token::ApplicationUserAuthorizationToken_WrongEnterTriesQuantity;
use entity::application_user_device::ApplicationUserDevice_Id;
use serde::Deserialize;
use serde::Serialize;
use macro_rules::r#enum;
use common_precedent::CommonPrecedent;

#[derive(Serialize, Deserialize)]
pub struct Incoming {
    pub application_user_device_id: ApplicationUserDevice_Id,
    pub application_user_email_or_application_user_nickname: String,
    pub application_user_password: String,
}

#[derive(Serialize, Deserialize)]
pub struct Outcoming {
    pub application_user_id: i64,
    pub verification_message_sent: bool,
    pub application_user_authorization_token_can_be_resent_from: ApplicationUserAuthorizationToken_CanBeResentFrom,
    pub application_user_authorization_token_wrong_enter_tries_quantity: ApplicationUserAuthorizationToken_WrongEnterTriesQuantity,
    pub application_user_authorization_token_wrong_enter_tries_quantity_limit: i16,
}

r#enum!(
    pub enum Precedent {
        CommonPrecedent::ApplicationUser_WrongEmailOrNicknameOrPassword,
    }
);