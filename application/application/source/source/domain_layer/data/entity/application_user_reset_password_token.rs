use super::application_user::ApplicationUser_Id;
use super::application_user_device::ApplicationUserDevice_Id;
use extern_crate::serde::Deserialize;
use extern_crate::serde::Serialize;
use std::borrow::Cow;

pub use self::CanBeResentFrom as ApplicationUserResetPasswordToken_CanBeResentFrom;
pub use self::ExpiresAt as ApplicationUserResetPasswordToken_ExpiresAt;
pub use self::IsApproved as ApplicationUserResetPasswordToken_IsApproved;
pub use self::Value as ApplicationUserResetPasswordToken_Value;
pub use self::WrongEnterTriesQuantity as ApplicationUserResetPasswordToken_WrongEnterTriesQuantity;

#[cfg_attr(
    feature = "manual_testing",
    derive(Serialize)
)]
#[derive(Deserialize)]
#[serde(crate = "extern_crate::serde")]
#[serde(transparent)]
pub struct Value(pub String);

#[derive(Clone, Copy)]
pub struct WrongEnterTriesQuantity(pub i16);

#[derive(Clone, Copy)]
pub struct IsApproved(pub bool);

#[derive(Clone, Copy)]
pub struct ExpiresAt(pub i64);

#[cfg_attr(
    feature = "manual_testing",
    derive(Deserialize)
)]
#[derive(Clone, Copy, Serialize)]
#[serde(crate = "extern_crate::serde")]
#[serde(transparent)]
pub struct CanBeResentFrom(pub i64);

pub struct ApplicationUserResetPasswordToken<'a> {
    pub application_user_id: ApplicationUser_Id,
    pub application_user_device_id: Cow<'a, ApplicationUserDevice_Id>,
    pub value: Value,
    pub wrong_enter_tries_quantity: WrongEnterTriesQuantity,
    pub is_approved: IsApproved,
    pub expires_at: ExpiresAt,
    pub can_be_resent_from: CanBeResentFrom,
}

impl<'a> ApplicationUserResetPasswordToken<'a> {
    pub const QUANTITY_OF_MINUTES_BEFORE_RESENDING: i64 = 1;
    pub const QUANTITY_OF_MINUTES_FOR_EXPIRATION: i64 = 10;
    pub const WRONG_ENTER_TRIES_QUANTITY_LIMIT: i16 = 3;
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