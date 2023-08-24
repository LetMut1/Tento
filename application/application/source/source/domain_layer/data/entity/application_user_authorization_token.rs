use super::application_user::ApplicationUser_Id;
use super::application_user_device::ApplicationUserDevice_Id;
use serde::Deserialize;
use serde::Serialize;
use std::borrow::Cow;

pub use self::CanBeResentFrom as ApplicationUserAuthorizationToken_CanBeResentFrom;
pub use self::ExpiresAt as ApplicationUserAuthorizationToken_ExpiresAt;
pub use self::Value as ApplicationUserAuthorizationToken_Value;
pub use self::WrongEnterTriesQuantity as ApplicationUserAuthorizationToken_WrongEnterTriesQuantity;

#[cfg_attr(
    feature = "manual_testing",
    derive(Serialize)
)]
#[derive(Deserialize)]
#[serde(transparent)]
pub struct Value(pub String);

impl Value {
    pub const REGULAR_EXPRESSION: &'static str = r#"^[0-9]{6}$"#;
}

#[cfg_attr(
    feature = "manual_testing",
    derive(Deserialize)
)]
#[derive(Clone, Copy, Serialize)]
#[serde(transparent)]
pub struct WrongEnterTriesQuantity(pub i16);

impl WrongEnterTriesQuantity {
    pub const LIMIT: i16 = 5;
}

#[derive(Clone, Copy)]
pub struct ExpiresAt(pub i64);

impl ExpiresAt {
    pub const QUANTITY_OF_MINUTES_FOR_EXPIRATION: i64 = 10;
}

#[cfg_attr(
    feature = "manual_testing",
    derive(Deserialize)
)]
#[derive(Clone, Copy, Serialize)]
#[serde(transparent)]
pub struct CanBeResentFrom(pub i64);

impl CanBeResentFrom {
    pub const QUANTITY_OF_MINUTES_BEFORE_RESENDING: i64 = 1;
}

pub struct ApplicationUserAuthorizationToken<'a> {
    pub application_user_id: ApplicationUser_Id,
    pub application_user_device_id: Cow<'a, ApplicationUserDevice_Id>,
    pub value: Value,
    pub wrong_enter_tries_quantity: WrongEnterTriesQuantity,
    pub expires_at: ExpiresAt,
    pub can_be_resent_from: CanBeResentFrom,
}

pub struct ApplicationUserAuthorizationToken1 {
    pub value: Value,
    pub wrong_enter_tries_quantity: WrongEnterTriesQuantity,
    pub expires_at: ExpiresAt,
    pub can_be_resent_from: CanBeResentFrom,
}

pub struct ApplicationUserAuthorizationToken2 {
    pub value: Value,
    pub wrong_enter_tries_quantity: WrongEnterTriesQuantity,
    pub expires_at: ExpiresAt,
}

pub struct ApplicationUserAuthorizationToken3 {
    pub can_be_resent_from: CanBeResentFrom,
}

pub struct ApplicationUserAuthorizationToken4 {
    pub wrong_enter_tries_quantity: WrongEnterTriesQuantity,
}

pub struct ApplicationUserAuthorizationToken5 {
    pub value: Value,
    pub expires_at: ExpiresAt,
    pub can_be_resent_from: CanBeResentFrom,
}
