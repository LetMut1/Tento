use super::application_user::ApplicationUser_Id;
use super::application_user_device::ApplicationUserDevice_Id;
use serde::Deserialize;
use serde::Serialize;
use std::marker::PhantomData;
use std::borrow::Cow;

pub use self::CanBeResentFrom as ApplicationUserAuthorizationToken_CanBeResentFrom;
pub use self::ExpiresAt as ApplicationUserAuthorizationToken_ExpiresAt;
pub use self::Value as ApplicationUserAuthorizationToken_Value;
pub use self::WrongEnterTriesQuantity as ApplicationUserAuthorizationToken_WrongEnterTriesQuantity;

pub struct ApplicationUserAuthorizationToken<'a> {
    pub application_user_id: i64,
    _application_user_id: PhantomData<ApplicationUser_Id>,

    pub application_user_device_id: Cow<'a, str>,
    _application_user_device_id: PhantomData<ApplicationUserDevice_Id>,

    pub value: String,
    _value: PhantomData<Value>,

    pub wrong_enter_tries_quantity: i16,
    _wrong_enter_tries_quantity: PhantomData<WrongEnterTriesQuantity>,

    pub expires_at: ExpiresAt,
    pub can_be_resent_from: CanBeResentFrom,
}

impl<'a> ApplicationUserAuthorizationToken<'a> {
    pub fn new(
        application_user_id: i64,
        application_user_device_id: Cow<'a, str>,
        value:String,
        wrong_enter_tries_quantity: i16,
        expires_at: ExpiresAt,
        can_be_resent_from: CanBeResentFrom,
    ) -> Self {
        return Self {
            application_user_id,
            _application_user_id: PhantomData,
            application_user_device_id,
            _application_user_device_id: PhantomData,
            value,
            _value: PhantomData,
            wrong_enter_tries_quantity,
            _wrong_enter_tries_quantity: PhantomData,
            expires_at,
            can_be_resent_from,
        };
    }
}

pub struct ApplicationUserAuthorizationToken1 {
    pub value: String,
    pub wrong_enter_tries_quantity: i16,
    pub expires_at: ExpiresAt,
    pub can_be_resent_from: CanBeResentFrom,
}

pub struct ApplicationUserAuthorizationToken2 {
    pub value: String,
    pub wrong_enter_tries_quantity: i16,
    pub expires_at: ExpiresAt,
}

pub struct ApplicationUserAuthorizationToken3 {
    pub can_be_resent_from: CanBeResentFrom,
}

pub struct ApplicationUserAuthorizationToken4 {
    pub wrong_enter_tries_quantity: i16,
}

pub struct ApplicationUserAuthorizationToken5 {
    pub value: String,
    pub expires_at: ExpiresAt,
    pub can_be_resent_from: CanBeResentFrom,
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