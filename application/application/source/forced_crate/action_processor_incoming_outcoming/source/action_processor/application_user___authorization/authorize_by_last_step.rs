use entity::application_user_authorization_token::ApplicationUserAuthorizationToken_WrongEnterTriesQuantity;
use serde::Deserialize;
use serde::Serialize;
use macro_rules::r#enum;
use common_precedent::CommonPrecedent;

#[derive(Serialize, Deserialize)]
pub struct Incoming {
    pub application_user_id: i64,
    pub application_user_device_id: String,
    pub application_user_authorization_token_value: String,
}

#[derive(Serialize, Deserialize)]
pub struct Outcoming {
    pub application_user_access_token_encrypted: String,
    pub application_user_access_refresh_token_encrypted: String,
}

r#enum!(
    pub enum Precedent {
        CommonPrecedent::ApplicationUserAuthorizationToken_NotFound,
        CommonPrecedent::ApplicationUserAuthorizationToken_AlreadyExpired,
        CommonPrecedent::ApplicationUserAuthorizationToken_WrongValue {
            application_user_authorization_token_wrong_enter_tries_quantity: ApplicationUserAuthorizationToken_WrongEnterTriesQuantity,
        },
        CommonPrecedent::ApplicationUser_NotFound,
    }
);