use entity::application_user::ApplicationUser_Email;
use entity::application_user_device::ApplicationUserDevice_Id;
use entity::application_user_registration_token::ApplicationUserRegistrationToken_CanBeResentFrom;
use serde::Deserialize;
use serde::Serialize;
use macro_rules::r#enum;
use common_precedent::CommonPrecedent;

#[derive(Serialize, Deserialize)]
pub struct Incoming {
    pub application_user_email: ApplicationUser_Email,
    pub application_user_device_id: ApplicationUserDevice_Id,
}

#[derive(Serialize, Deserialize)]
pub struct Outcoming {
    pub application_user_registration_token_can_be_resent_from: ApplicationUserRegistrationToken_CanBeResentFrom,
}

r#enum!(
    pub enum Precedent {
        CommonPrecedent::ApplicationUserRegistrationToken_NotFound,
        CommonPrecedent::ApplicationUserRegistrationToken_AlreadyExpired,
        CommonPrecedent::ApplicationUserRegistrationToken_AlreadyApproved,
        CommonPrecedent::ApplicationUserRegistrationToken_TimeToResendHasNotCome,
    }
);