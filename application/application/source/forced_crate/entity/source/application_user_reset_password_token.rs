use super::application_user::ApplicationUser_Id;
use super::application_user_device::ApplicationUserDevice_Id;
use serde::Deserialize;
use serde::Serialize;
use std::borrow::Cow;

pub use self::CanBeResentFrom as ApplicationUserResetPasswordToken_CanBeResentFrom;
pub use self::ExpiresAt as ApplicationUserResetPasswordToken_ExpiresAt;
pub use self::IsApproved as ApplicationUserResetPasswordToken_IsApproved;
pub use self::Value as ApplicationUserResetPasswordToken_Value;
pub use self::WrongEnterTriesQuantity as ApplicationUserResetPasswordToken_WrongEnterTriesQuantity;

#[derive(Serialize, Deserialize)]
#[serde(transparent)]
pub struct Value(pub String);

impl Value {
    pub const REGULAR_EXPRESSION: &'static str = r#"^[0-9]{6}$"#;
}

#[derive(Clone, Copy, Serialize, Deserialize)]
#[serde(transparent)]
pub struct WrongEnterTriesQuantity(pub i16);

impl WrongEnterTriesQuantity {
    pub const LIMIT: i16 = 3;
}

#[derive(Clone, Copy)]
pub struct IsApproved(pub bool);

#[derive(Clone, Copy)]
pub struct ExpiresAt(pub i64);

impl ExpiresAt {
    pub const QUANTITY_OF_MINUTES_FOR_EXPIRATION: i64 = 10;
}

#[derive(Clone, Copy, Serialize, Deserialize)]
#[serde(transparent)]
pub struct CanBeResentFrom(pub i64);

impl CanBeResentFrom {
    pub const QUANTITY_OF_MINUTES_BEFORE_RESENDING: i64 = 1;
}

pub struct ApplicationUserResetPasswordToken<'a> {
    pub application_user_id: ApplicationUser_Id,
    pub application_user_device_id: Cow<'a, ApplicationUserDevice_Id>,
    pub value: Value,
    pub wrong_enter_tries_quantity: WrongEnterTriesQuantity,
    pub is_approved: IsApproved,
    pub expires_at: ExpiresAt,
    pub can_be_resent_from: CanBeResentFrom,
}

pub struct ApplicationUserResetPasswordToken1 {
    pub value: Value,
    pub wrong_enter_tries_quantity: WrongEnterTriesQuantity,
    pub is_approved: IsApproved,
    pub expires_at: ExpiresAt,
    pub can_be_resent_from: CanBeResentFrom,
}

pub struct ApplicationUserResetPasswordToken2 {
    pub can_be_resent_from: CanBeResentFrom,
}

pub struct ApplicationUserResetPasswordToken3 {
    pub value: Value,
    pub wrong_enter_tries_quantity: WrongEnterTriesQuantity,
    pub is_approved: IsApproved,
    pub expires_at: ExpiresAt,
}

pub struct ApplicationUserResetPasswordToken4 {
    pub wrong_enter_tries_quantity: WrongEnterTriesQuantity,
}

pub struct ApplicationUserResetPasswordToken5 {
    pub is_approved: IsApproved,
}

pub struct ApplicationUserResetPasswordToken6 {
    pub value: Value,
    pub is_approved: IsApproved,
    pub expires_at: ExpiresAt,
    pub can_be_resent_from: CanBeResentFrom,
}
