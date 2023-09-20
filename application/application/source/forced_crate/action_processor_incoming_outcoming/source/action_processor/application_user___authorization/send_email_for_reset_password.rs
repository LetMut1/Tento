use entity::application_user::ApplicationUser_Id;
use entity::application_user_device::ApplicationUserDevice_Id;
use entity::application_user_reset_password_token::ApplicationUserResetPasswordToken_CanBeResentFrom;
use serde::Deserialize;
use macro_rules::r#enum;
use common_precedent::CommonPrecedent;
use serde::Serialize;

#[derive(Serialize, Deserialize)]
pub struct Incoming {
    pub application_user_id: ApplicationUser_Id,
    pub application_user_device_id: ApplicationUserDevice_Id,
}

#[derive(Serialize, Deserialize)]
pub struct Outcoming {
    pub application_user_registration_token_can_be_resent_from: ApplicationUserResetPasswordToken_CanBeResentFrom,
}

r#enum!(
    pub enum Precedent {
        CommonPrecedent::ApplicationUser_NotFound,
        CommonPrecedent::ApplicationUserResetPasswordToken_NotFound,
        CommonPrecedent::ApplicationUserResetPasswordToken_AlreadyExpired,
        CommonPrecedent::ApplicationUserResetPasswordToken_AlreadyApproved,
        CommonPrecedent::ApplicationUserResetPasswordToken_TimeToResendHasNotCome,
    }
);