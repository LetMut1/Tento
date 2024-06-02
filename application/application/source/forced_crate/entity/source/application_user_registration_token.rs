use super::application_user::ApplicationUser_Email;
use super::application_user_device::ApplicationUserDevice_Id;
use serde::Deserialize;
use serde::Serialize;
use std::marker::PhantomData;
use std::borrow::Cow;

pub use self::CanBeResentFrom as ApplicationUserRegistrationToken_CanBeResentFrom;
pub use self::ExpiresAt as ApplicationUserRegistrationToken_ExpiresAt;
pub use self::IsApproved as ApplicationUserRegistrationToken_IsApproved;
pub use self::Value as ApplicationUserRegistrationToken_Value;
pub use self::WrongEnterTriesQuantity as ApplicationUserRegistrationToken_WrongEnterTriesQuantity;

pub struct ApplicationUserRegistrationToken<'a> {
    pub application_user_email: Cow<'a, str>,
    _application_user_email: PhantomData<ApplicationUser_Email>,

    pub application_user_device_id: Cow<'a, str>,
    _application_user_device_id: PhantomData<ApplicationUserDevice_Id>,

    pub value: String,
    _value: PhantomData<Value>,

    pub wrong_enter_tries_quantity: i16,
    _wrong_enter_tries_quantity: PhantomData<WrongEnterTriesQuantity>,

    pub is_approved: bool,
    _is_approved: PhantomData<IsApproved>,

    pub expires_at: i64,
    _expires_at: PhantomData<ExpiresAt>,

    pub can_be_resent_from: i64,
    _can_be_resent_from: PhantomData<CanBeResentFrom>
}

impl<'a> ApplicationUserRegistrationToken<'a> {
    pub fn new(
        application_user_email: Cow<'a, str>,
        application_user_device_id: Cow<'a, str>,
        value: String,
        wrong_enter_tries_quantity: i16,
        is_approved: bool,
        expires_at: i64,
        can_be_resent_from: i64,
    ) -> Self {
        return Self {
            application_user_email,
            _application_user_email: PhantomData,
            application_user_device_id,
            _application_user_device_id: PhantomData,
            value,
            _value: PhantomData,
            wrong_enter_tries_quantity,
            _wrong_enter_tries_quantity: PhantomData,
            is_approved,
            _is_approved: PhantomData,
            expires_at,
            _expires_at: PhantomData,
            can_be_resent_from,
            _can_be_resent_from: PhantomData,
        };
    }
}

#[derive(Serialize, Deserialize)]
pub struct Value;

impl Value {
    pub const REGULAR_EXPRESSION: &'static str = r#"^[0-9]{6}$"#;
}

#[derive(Serialize, Deserialize)]
pub struct WrongEnterTriesQuantity;

impl WrongEnterTriesQuantity {
    pub const LIMIT: i16 = 5;
}

pub struct IsApproved;

pub struct ExpiresAt;

impl ExpiresAt {
    pub const QUANTITY_OF_MINUTES_FOR_EXPIRATION: i64 = 60 * 3;
}

#[derive(Serialize, Deserialize)]
pub struct CanBeResentFrom;

impl CanBeResentFrom {
    pub const QUANTITY_OF_MINUTES_BEFORE_RESENDING: i64 = 1;
}

pub struct ApplicationUserRegistrationToken1 {
    pub value: String,
    pub wrong_enter_tries_quantity: i16,
    pub is_approved: bool,
    pub expires_at: i64,
    pub can_be_resent_from: i64,
}

pub struct ApplicationUserRegistrationToken2 {
    pub can_be_resent_from: i64,
}

pub struct ApplicationUserRegistrationToken3 {
    pub value: String,
    pub wrong_enter_tries_quantity: i16,
    pub is_approved: bool,
    pub expires_at: i64,
}

pub struct ApplicationUserRegistrationToken4 {
    pub wrong_enter_tries_quantity: i16,
}

pub struct ApplicationUserRegistrationToken5 {
    pub is_approved: bool,
}

pub struct ApplicationUserRegistrationToken6 {
    pub value: String,
    pub is_approved: bool,
    pub expires_at: i64,
    pub can_be_resent_from: i64,
}
