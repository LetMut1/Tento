use entity::application_user::ApplicationUser_Id;
use entity::application_user_access_refresh_token_encrypted::ApplicationUserAccessRefreshTokenEncrypted;
use entity::application_user_access_token_encrypted::ApplicationUserAccessTokenEncrypted;
use entity::application_user_authorization_token::ApplicationUserAuthorizationToken_Value;
use entity::application_user_device::ApplicationUserDevice_Id;
use entity::application_user_authorization_token::ApplicationUserAuthorizationToken_WrongEnterTriesQuantity;
use serde::Deserialize;
use serde::Serialize;
use macro_rules::r#enum;
use common_precedent::CommonPrecedent;

#[derive(Serialize, Deserialize)]
pub struct Incoming {
    pub application_user_id: ApplicationUser_Id,
    pub application_user_device_id: ApplicationUserDevice_Id,
    pub application_user_authorization_token_value: ApplicationUserAuthorizationToken_Value,
}

#[derive(Serialize, Deserialize)]
pub struct Outcoming {
    pub application_user_access_token_encrypted: ApplicationUserAccessTokenEncrypted,
    pub application_user_access_refresh_token_encrypted: ApplicationUserAccessRefreshTokenEncrypted,
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