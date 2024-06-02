use serde::Deserialize;
use serde::Serialize;
use macro_rules::r#enum;
use common_precedent::CommonPrecedent;

#[derive(Serialize, Deserialize)]
pub struct Incoming {
    pub application_user_device_id: String,
    pub application_user_nickname: String,
    pub application_user_password: String,
    pub application_user_email: String,
    pub application_user_registration_token_value: String,
}

#[derive(Serialize, Deserialize)]
pub struct Outcoming {
    pub application_user_access_token_encrypted: String,
    pub application_user_access_refresh_token_encrypted: String,
}

r#enum!(
    pub enum Precedent {
        CommonPrecedent::ApplicationUser_NicknameAlreadyExist,
        CommonPrecedent::ApplicationUser_EmailAlreadyExist,
        CommonPrecedent::ApplicationUserRegistrationToken_NotFound,
        CommonPrecedent::ApplicationUserRegistrationToken_AlreadyExpired,
        CommonPrecedent::ApplicationUserRegistrationToken_IsNotApproved,
        CommonPrecedent::ApplicationUserRegistrationToken_WrongValue,
    }
);