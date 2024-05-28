use entity::application_user::ApplicationUser_Nickname;
use entity::application_user::ApplicationUser_Password;
use entity::application_user_access_refresh_token_encrypted::ApplicationUserAccessRefreshTokenEncrypted;
use entity::application_user_access_token_encrypted::ApplicationUserAccessTokenEncrypted;
use entity::application_user_device::ApplicationUserDevice_Id;
use entity::application_user_registration_token::ApplicationUserRegistrationToken_Value;
use serde::Deserialize;
use serde::Serialize;
use macro_rules::r#enum;
use common_precedent::CommonPrecedent;

#[derive(Serialize, Deserialize)]
pub struct Incoming {
    pub application_user_device_id: ApplicationUserDevice_Id,
    pub application_user_nickname: ApplicationUser_Nickname,
    pub application_user_password: ApplicationUser_Password,
    pub application_user_email: String,
    pub application_user_registration_token_value: ApplicationUserRegistrationToken_Value,
}

#[derive(Serialize, Deserialize)]
pub struct Outcoming {
    pub application_user_access_token_encrypted: ApplicationUserAccessTokenEncrypted,
    pub application_user_access_refresh_token_encrypted: ApplicationUserAccessRefreshTokenEncrypted,
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