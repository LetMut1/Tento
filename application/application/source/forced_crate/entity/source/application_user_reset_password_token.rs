use super::application_user::ApplicationUser_Id;
use super::application_user_device::ApplicationUserDevice_Id;
use std::borrow::Cow;
use std::marker::PhantomData;

pub use self::CanBeResentFrom as ApplicationUserResetPasswordToken_CanBeResentFrom;
pub use self::ExpiresAt as ApplicationUserResetPasswordToken_ExpiresAt;
pub use self::IsApproved as ApplicationUserResetPasswordToken_IsApproved;
pub use self::Value as ApplicationUserResetPasswordToken_Value;
pub use self::WrongEnterTriesQuantity as ApplicationUserResetPasswordToken_WrongEnterTriesQuantity;

pub struct ApplicationUserResetPasswordToken<'a> {
    pub application_user_id: i64,
    _application_user_id: PhantomData<ApplicationUser_Id>,

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
    _can_be_resent_from: PhantomData<CanBeResentFrom>,
}

impl<'a> ApplicationUserResetPasswordToken<'a> {
    pub fn new(
        application_user_id: i64,
        application_user_device_id: Cow<'a, str>,
        value: String,
        wrong_enter_tries_quantity: i16,
        is_approved: bool,
        expires_at: i64,
        can_be_resent_from: i64,
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
            is_approved,
            _is_approved: PhantomData,
            expires_at,
            _expires_at: PhantomData,
            can_be_resent_from,
            _can_be_resent_from: PhantomData,
        };
    }
}

pub struct ApplicationUserResetPasswordToken1 {
    pub value: String,
    pub wrong_enter_tries_quantity: i16,
    pub is_approved: bool,
    pub expires_at: i64,
    pub can_be_resent_from: i64,
}

pub struct ApplicationUserResetPasswordToken2 {
    pub can_be_resent_from: i64,
}

pub struct ApplicationUserResetPasswordToken3 {
    pub value: String,
    pub wrong_enter_tries_quantity: i16,
    pub is_approved: bool,
    pub expires_at: i64,
}

pub struct ApplicationUserResetPasswordToken4 {
    pub wrong_enter_tries_quantity: i16,
}

pub struct ApplicationUserResetPasswordToken5 {
    pub is_approved: bool,
}

pub struct ApplicationUserResetPasswordToken6 {
    pub value: String,
    pub is_approved: bool,
    pub expires_at: i64,
    pub can_be_resent_from: i64,
}

pub struct Value;

impl Value {
    pub const REGULAR_EXPRESSION: &'static str = r#"^[0-9]{6}$"#;
}

pub struct WrongEnterTriesQuantity;

impl WrongEnterTriesQuantity {
    pub const LIMIT: i16 = 3;
}

pub struct IsApproved;

pub struct ExpiresAt;

impl ExpiresAt {
    pub const QUANTITY_OF_MINUTES_FOR_EXPIRATION: i64 = 10;
}

pub struct CanBeResentFrom;

impl CanBeResentFrom {
    pub const QUANTITY_OF_MINUTES_BEFORE_RESENDING: i64 = 1;
}