use super::application_user::ApplicationUser_Email;
use super::application_user_device::ApplicationUserDevice_Id;
use extern_crate::serde::Deserialize;
use extern_crate::serde::Serialize;
use std::borrow::Cow;

pub use self::CanBeResentFrom as ApplicationUserRegistrationToken_CanBeResentFrom;
pub use self::ExpiresAt as ApplicationUserRegistrationToken_ExpiresAt;
pub use self::IsApproved as ApplicationUserRegistrationToken_IsApproved;
pub use self::Value as ApplicationUserRegistrationToken_Value;
pub use self::WrongEnterTriesQuantity as ApplicationUserRegistrationToken_WrongEnterTriesQuantity;

#[cfg_attr(
    feature = "manual_testing",
    derive(Serialize)
)]
#[derive(Deserialize)]
#[serde(crate = "extern_crate::serde")]
#[serde(transparent)]
pub struct Value(String);

impl Value {
    pub fn new(inner: String) -> Self {
        return Self(inner);
    }

    pub fn get<'a>(&'a self) -> &'a str {
        return self.0.as_str();
    }
}

#[derive(Clone, Copy)]
pub struct WrongEnterTriesQuantity(i16);

impl WrongEnterTriesQuantity {
    pub fn new(inner: i16) -> Self {
        return Self(inner);
    }

    pub fn get<'a>(&'a self) -> i16 {
        return self.0;
    }
}

#[derive(Clone, Copy)]
pub struct IsApproved(bool);

impl IsApproved {
    pub fn new(inner: bool) -> Self {
        return Self(inner);
    }

    pub fn get<'a>(&'a self) -> bool {
        return self.0;
    }
}

#[derive(Clone, Copy)]
pub struct ExpiresAt(i64);

impl ExpiresAt {
    pub fn new(inner: i64) -> Self {
        return Self(inner);
    }

    pub fn get<'a>(&'a self) -> i64 {
        return self.0;
    }
}

#[derive(Clone, Copy)]
#[cfg_attr(
    feature = "manual_testing",
    derive(Deserialize)
)]
#[derive(Serialize)]
#[serde(crate = "extern_crate::serde")]
#[serde(transparent)]
pub struct CanBeResentFrom(i64);

impl CanBeResentFrom {
    pub fn new(inner: i64) -> Self {
        return Self(inner);
    }

    pub fn get<'a>(&'a self) -> i64 {
        return self.0;
    }
}

pub struct ApplicationUserRegistrationToken<'a> {
    pub application_user_email: Cow<'a, ApplicationUser_Email>,
    pub application_user_device_id: Cow<'a, ApplicationUserDevice_Id>,
    pub value: Value,
    pub wrong_enter_tries_quantity: WrongEnterTriesQuantity,
    pub is_approved: IsApproved,
    pub expires_at: ExpiresAt,
    pub can_be_resent_from: CanBeResentFrom,
}

impl<'a> ApplicationUserRegistrationToken<'a> {
    pub const QUANTITY_OF_MINUTES_BEFORE_RESENDING: i64 = 1;
    pub const QUANTITY_OF_MINUTES_FOR_EXPIRATION: i64 = 60 * 3;
    pub const WRONG_ENTER_TRIES_QUANTITY_LIMIT: i16 = 5;
}

pub struct ApplicationUserRegistrationToken1 {
    pub value: Value,
    pub wrong_enter_tries_quantity: WrongEnterTriesQuantity,
    pub is_approved: IsApproved,
    pub expires_at: ExpiresAt,
    pub can_be_resent_from: CanBeResentFrom,
}

pub struct ApplicationUserRegistrationToken2 {
    pub can_be_resent_from: CanBeResentFrom,
}

pub struct ApplicationUserRegistrationToken3 {
    pub value: Value,
    pub wrong_enter_tries_quantity: WrongEnterTriesQuantity,
    pub is_approved: IsApproved,
    pub expires_at: ExpiresAt,
}

pub struct ApplicationUserRegistrationToken4 {
    pub wrong_enter_tries_quantity: WrongEnterTriesQuantity,
}

pub struct ApplicationUserRegistrationToken5 {
    pub is_approved: IsApproved,
}

pub struct ApplicationUserRegistrationToken6 {
    pub value: Value,
    pub is_approved: IsApproved,
    pub expires_at: ExpiresAt,
    pub can_be_resent_from: CanBeResentFrom,
}