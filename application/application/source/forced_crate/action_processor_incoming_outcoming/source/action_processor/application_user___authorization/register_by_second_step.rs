use entity::application_user_registration_token::ApplicationUserRegistrationToken_Value;
use entity::application_user_registration_token::ApplicationUserRegistrationToken_WrongEnterTriesQuantity;
use serde::Deserialize;
use macro_rules::r#enum;
use common_precedent::CommonPrecedent;
use serde::Serialize;

#[derive(Serialize, Deserialize)]
pub struct Incoming {
    pub application_user_email: String,
    pub application_user_device_id: String,
    pub application_user_registration_token_value: ApplicationUserRegistrationToken_Value,
}

r#enum!(
    pub enum Precedent {
        CommonPrecedent::ApplicationUserRegistrationToken_NotFound,
        CommonPrecedent::ApplicationUserRegistrationToken_AlreadyExpired,
        CommonPrecedent::ApplicationUserRegistrationToken_AlreadyApproved,
        CommonPrecedent::ApplicationUserRegistrationToken_WrongValue {
            application_user_registration_token_wrong_enter_tries_quantity: ApplicationUserRegistrationToken_WrongEnterTriesQuantity,
        },
    }
);