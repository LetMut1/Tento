use entity::application_user_registration_token::ApplicationUserRegistrationToken_CanBeResentFrom;
use entity::application_user_registration_token::ApplicationUserRegistrationToken_WrongEnterTriesQuantity;
use serde::Deserialize;
use serde::Serialize;
use macro_rules::r#enum;
use common_precedent::CommonPrecedent;

#[derive(Serialize, Deserialize)]
pub struct Incoming {
    pub application_user_email: String,
    pub application_user_device_id: String,
}

#[derive(Serialize, Deserialize)]
pub struct Outcoming {
    pub verification_message_sent: bool,
    pub application_user_registration_token_can_be_resent_from: ApplicationUserRegistrationToken_CanBeResentFrom,
    pub application_user_registration_token_wrong_enter_tries_quantity: ApplicationUserRegistrationToken_WrongEnterTriesQuantity,
    pub application_user_registration_token_wrong_enter_tries_quantity_limit: i16,
}

r#enum!(
    pub enum Precedent {
        CommonPrecedent::ApplicationUser_EmailAlreadyExist,
    }
);