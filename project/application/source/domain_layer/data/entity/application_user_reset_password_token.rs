use crate::domain_layer::functionality::service::getter::GetterDELETE;
use std::borrow::Cow;
use std::marker::PhantomData;
use super::application_user_device::ApplicationUserDevice_Id;
use super::application_user::ApplicationUser_Id;

pub use self::Value as ApplicationUserResetPasswordToken_Value;
pub use self::WrongEnterTriesQuantity as ApplicationUserResetPasswordToken_WrongEnterTriesQuantity;
pub use self::IsApproved as ApplicationUserResetPasswordToken_IsApproved;
pub use self::ExpiresAt as ApplicationUserResetPasswordToken_ExpiresAt;
pub use self::CanBeResentFrom as ApplicationUserResetPasswordToken_CanBeResentFrom;

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
pub struct CanBeResentFrom(i64);

impl CanBeResentFrom {
    pub fn new(inner: i64) -> Self {
        return Self(inner);
    }

    pub fn get<'a>(&'a self) -> i64 {
        return self.0;
    }
}

pub struct ApplicationUserResetPasswordToken<'a> {
    application_user_id: i64,
    _application_user_id: PhantomData<ApplicationUser_Id>,

    application_user_device_id: Cow<'a, str>,
    _application_user_device_id: PhantomData<ApplicationUserDevice_Id>,

    value: String,
    _value: PhantomData<Value>,

    wrong_enter_tries_quantity: i16,
    _wrong_enter_tries_quantity: PhantomData<WrongEnterTriesQuantity>,

    is_approved: bool,
    _is_approved: PhantomData<IsApproved>,

    expires_at: i64,
    _expires_at: PhantomData<ExpiresAt>,

    can_be_resent_from: i64,
    _can_be_resent_from: PhantomData<CanBeResentFrom>
}

impl<'a> ApplicationUserResetPasswordToken<'a> {
    pub const QUANTITY_OF_MINUTES_BEFORE_RESENDING: i64 = 1;
    pub const QUANTITY_OF_MINUTES_FOR_EXPIRATION: i64 = 10;
    pub const WRONG_ENTER_TRIES_QUANTITY_LIMIT: i16 = 3;

    pub fn new(
        application_user_id: i64,
        application_user_device_id: Cow<'a, str>,
        value: String,
        wrong_enter_tries_quantity: i16,
        is_approved: bool,
        expires_at: i64,
        can_be_resent_from: i64
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
            _can_be_resent_from: PhantomData
        };
    }

    pub fn get_application_user_id<'b>(&'b self) -> i64 {
        return self.application_user_id;
    }

    pub fn get_application_user_device_id<'b>(&'b self) -> &'b str {
        return self.application_user_device_id.as_ref();
    }

    pub fn get_value<'b>(&'b self) -> &'b str {
        return self.value.as_str();
    }

    pub fn get_wrong_enter_tries_quantity<'b>(&'b self) -> i16 {
        return self.wrong_enter_tries_quantity;
    }

    pub fn get_is_approved<'b>(&'b self) -> bool {
        return self.is_approved;
    }

    pub fn get_expires_at<'b>(&'b self) -> i64 {
        return self.expires_at;
    }

    pub fn get_can_be_resent_from<'b>(&'b self) -> i64 {
        return self.can_be_resent_from;
    }

    pub fn set_value<'b>(&'b mut self, value: String) -> &'b mut Self {
        self.value = value;

        return self;
    }

    pub fn set_wrong_enter_tries_quantity<'b>(&'b mut self, wrong_enter_tries_quantity: i16) -> &'b mut Self {
        self.wrong_enter_tries_quantity = wrong_enter_tries_quantity;

        return self;
    }

    pub fn set_is_approved<'b>(&'b mut self, is_approved: bool) -> &'b mut Self {
        self.is_approved = is_approved;

        return self;
    }

    pub fn set_expires_at<'b>(&'b mut self, expires_at: i64) -> &'b mut Self {
        self.expires_at = expires_at;

        return self;
    }

    pub fn set_can_be_resent_from<'b>(&'b mut self, can_be_resent_from: i64) -> &'b mut Self {
        self.can_be_resent_from = can_be_resent_from;

        return self;
    }
}

impl<'a> GetterDELETE<&'a Self, ApplicationUser_Id, i64> for ApplicationUserResetPasswordToken<'_> {
    fn get(subject: &'a Self) -> i64 {
        return subject.application_user_id;
    }
}

impl<'a, 'b: 'a> GetterDELETE<&'a Self, ApplicationUserDevice_Id, &'a str> for ApplicationUserResetPasswordToken<'b> {
    fn get(subject: &'a Self) -> &'a str {
        return subject.application_user_device_id.as_ref();
    }
}

impl<'a> GetterDELETE<&'a Self, Value, &'a str> for ApplicationUserResetPasswordToken<'_> {
    fn get(subject: &'a Self) -> &'a str {
        return subject.value.as_str();
    }
}

impl<'a> GetterDELETE<&'a Self, WrongEnterTriesQuantity, i16> for ApplicationUserResetPasswordToken<'_> {
    fn get(subject: &'a Self) -> i16 {
        return subject.wrong_enter_tries_quantity;
    }
}

impl<'a> GetterDELETE<&'a Self, IsApproved, bool> for ApplicationUserResetPasswordToken<'_> {
    fn get(subject: &'a Self) -> bool {
        return subject.is_approved;
    }
}

impl<'a> GetterDELETE<&'a Self, ExpiresAt, i64> for ApplicationUserResetPasswordToken<'_> {
    fn get(subject: &'a Self) -> i64 {
        return subject.expires_at;
    }
}

impl<'a> GetterDELETE<&'a Self, CanBeResentFrom, i64> for ApplicationUserResetPasswordToken<'_> {
    fn get(subject: &'a Self) -> i64 {
        return subject.can_be_resent_from;
    }
}

pub struct ApplicationUserResetPasswordToken_1 {
    value: String,
    _value: PhantomData<Value>,

    wrong_enter_tries_quantity: i16,
    _wrong_enter_tries_quantity: PhantomData<WrongEnterTriesQuantity>,

    is_approved: bool,
    _is_approved: PhantomData<IsApproved>,

    expires_at: i64,
    _expires_at: PhantomData<ExpiresAt>,

    can_be_resent_from: i64,
    _can_be_resent_from: PhantomData<CanBeResentFrom>
}

impl<'a> ApplicationUserResetPasswordToken_1 {
    pub fn new(
        value: String,
        wrong_enter_tries_quantity: i16,
        is_approved: bool,
        expires_at: i64,
        can_be_resent_from: i64
    ) -> Self {
        return Self {
            value,
            _value: PhantomData,
            wrong_enter_tries_quantity,
            _wrong_enter_tries_quantity: PhantomData,
            is_approved,
            _is_approved: PhantomData,
            expires_at,
            _expires_at: PhantomData,
            can_be_resent_from,
            _can_be_resent_from: PhantomData
        };
    }

    pub fn get_value<'b>(&'b self) -> &'b str {
        return self.value.as_str();
    }

    pub fn get_wrong_enter_tries_quantity<'b>(&'b self) -> i16 {
        return self.wrong_enter_tries_quantity;
    }

    pub fn get_is_approved<'b>(&'b self) -> bool {
        return self.is_approved;
    }

    pub fn get_expires_at<'b>(&'b self) -> i64 {
        return self.expires_at;
    }

    pub fn get_can_be_resent_from<'b>(&'b self) -> i64 {
        return self.can_be_resent_from;
    }

    pub fn set_value<'b>(&'b mut self, value: String) -> &'b mut Self {
        self.value = value;

        return self;
    }

    pub fn set_wrong_enter_tries_quantity<'b>(&'b mut self, wrong_enter_tries_quantity: i16) -> &'b mut Self {
        self.wrong_enter_tries_quantity = wrong_enter_tries_quantity;

        return self;
    }

    pub fn set_is_approved<'b>(&'b mut self, is_approved: bool) -> &'b mut Self {
        self.is_approved = is_approved;

        return self;
    }

    pub fn set_expires_at<'b>(&'b mut self, expires_at: i64) -> &'b mut Self {
        self.expires_at = expires_at;

        return self;
    }

    pub fn set_can_be_resent_from<'b>(&'b mut self, can_be_resent_from: i64) -> &'b mut Self {
        self.can_be_resent_from = can_be_resent_from;

        return self;
    }
}

impl<'a> GetterDELETE<&'a Self, Value, &'a str> for ApplicationUserResetPasswordToken_1 {
    fn get(subject: &'a Self) -> &'a str {
        return subject.value.as_str();
    }
}

impl<'a> GetterDELETE<&'a Self, WrongEnterTriesQuantity, i16> for ApplicationUserResetPasswordToken_1 {
    fn get(subject: &'a Self) -> i16 {
        return subject.wrong_enter_tries_quantity;
    }
}

impl<'a> GetterDELETE<&'a Self, IsApproved, bool> for ApplicationUserResetPasswordToken_1 {
    fn get(subject: &'a Self) -> bool {
        return subject.is_approved;
    }
}

impl<'a> GetterDELETE<&'a Self, ExpiresAt, i64> for ApplicationUserResetPasswordToken_1 {
    fn get(subject: &'a Self) -> i64 {
        return subject.expires_at;
    }
}

impl<'a> GetterDELETE<&'a Self, CanBeResentFrom, i64> for ApplicationUserResetPasswordToken_1 {
    fn get(subject: &'a Self) -> i64 {
        return subject.can_be_resent_from;
    }
}

pub struct ApplicationUserResetPasswordToken_2 {
    can_be_resent_from: i64,
    _can_be_resent_from: PhantomData<CanBeResentFrom>
}

pub struct ApplicationUserResetPasswordToken_3 {
    value: String,
    _value: PhantomData<Value>,

    wrong_enter_tries_quantity: i16,
    _wrong_enter_tries_quantity: PhantomData<WrongEnterTriesQuantity>,

    is_approved: bool,
    _is_approved: PhantomData<IsApproved>,

    expires_at: i64,
    _expires_at: PhantomData<ExpiresAt>
}

impl ApplicationUserResetPasswordToken_3 {
    pub fn new(
        value: String,
        wrong_enter_tries_quantity: i16,
        is_approved: bool,
        expires_at: i64
    ) -> Self {
        return Self {
            value,
            _value: PhantomData,
            wrong_enter_tries_quantity,
            _wrong_enter_tries_quantity: PhantomData,
            is_approved,
            _is_approved: PhantomData,
            expires_at,
            _expires_at: PhantomData
        };
    }

    pub fn get_value<'b>(&'b self) -> &'b str {
        return self.value.as_str();
    }

    pub fn get_wrong_enter_tries_quantity<'b>(&'b self) -> i16 {
        return self.wrong_enter_tries_quantity;
    }

    pub fn get_is_approved<'b>(&'b self) -> bool {
        return self.is_approved;
    }

    pub fn get_expires_at<'b>(&'b self) -> i64 {
        return self.expires_at;
    }

    pub fn set_value<'b>(&'b mut self, value: String) -> &'b mut Self {
        self.value = value;

        return self;
    }

    pub fn set_wrong_enter_tries_quantity<'b>(&'b mut self, wrong_enter_tries_quantity: i16) -> &'b mut Self {
        self.wrong_enter_tries_quantity = wrong_enter_tries_quantity;

        return self;
    }

    pub fn set_is_approved<'b>(&'b mut self, is_approved: bool) -> &'b mut Self {
        self.is_approved = is_approved;

        return self;
    }

    pub fn set_expires_at<'b>(&'b mut self, expires_at: i64) -> &'b mut Self {
        self.expires_at = expires_at;

        return self;
    }
}

impl<'a> GetterDELETE<&'a Self, Value, &'a str> for ApplicationUserResetPasswordToken_3 {
    fn get(subject: &'a Self) -> &'a str {
        return subject.value.as_str();
    }
}

impl<'a> GetterDELETE<&'a Self, WrongEnterTriesQuantity, i16> for ApplicationUserResetPasswordToken_3 {
    fn get(subject: &'a Self) -> i16 {
        return subject.wrong_enter_tries_quantity;
    }
}

impl<'a> GetterDELETE<&'a Self, IsApproved, bool> for ApplicationUserResetPasswordToken_3 {
    fn get(subject: &'a Self) -> bool {
        return subject.is_approved;
    }
}

impl<'a> GetterDELETE<&'a Self, ExpiresAt, i64> for ApplicationUserResetPasswordToken_3 {
    fn get(subject: &'a Self) -> i64 {
        return subject.expires_at;
    }
}

pub struct ApplicationUserResetPasswordToken_4 {
    wrong_enter_tries_quantity: i16,
    _wrong_enter_tries_quantity: PhantomData<WrongEnterTriesQuantity>,
}

pub struct ApplicationUserResetPasswordToken_5 {
    is_approved: bool,
    _is_approved: PhantomData<IsApproved>
}

pub struct ApplicationUserResetPasswordToken_6 {
    value: String,
    _value: PhantomData<Value>,

    is_approved: bool,
    _is_approved: PhantomData<IsApproved>,

    expires_at: i64,
    _expires_at: PhantomData<ExpiresAt>,

    can_be_resent_from: i64,
    _can_be_resent_from: PhantomData<CanBeResentFrom>
}

impl<'a> ApplicationUserResetPasswordToken_6 {
    pub fn new(
        value: String,
        is_approved: bool,
        expires_at: i64,
        can_be_resent_from: i64
    ) -> Self {
        return Self {
            value,
            _value: PhantomData,
            is_approved,
            _is_approved: PhantomData,
            expires_at,
            _expires_at: PhantomData,
            can_be_resent_from,
            _can_be_resent_from: PhantomData
        };
    }

    pub fn get_value<'b>(&'b self) -> &'b str {
        return self.value.as_str();
    }

    pub fn get_is_approved<'b>(&'b self) -> bool {
        return self.is_approved;
    }

    pub fn get_expires_at<'b>(&'b self) -> i64 {
        return self.expires_at;
    }

    pub fn get_can_be_resent_from<'b>(&'b self) -> i64 {
        return self.can_be_resent_from;
    }

    pub fn set_can_be_resent_from<'b>(&'b mut self, can_be_resent_from: i64) -> &'b mut Self {
        self.can_be_resent_from = can_be_resent_from;

        return self;
    }
}

impl<'a> GetterDELETE<&'a Self, Value, &'a str> for ApplicationUserResetPasswordToken_6 {
    fn get(subject: &'a Self) -> &'a str {
        return subject.value.as_str();
    }
}

impl<'a> GetterDELETE<&'a Self, IsApproved, bool> for ApplicationUserResetPasswordToken_6 {
    fn get(subject: &'a Self) -> bool {
        return subject.is_approved;
    }
}

impl<'a> GetterDELETE<&'a Self, ExpiresAt, i64> for ApplicationUserResetPasswordToken_6 {
    fn get(subject: &'a Self) -> i64 {
        return subject.expires_at;
    }
}

impl<'a> GetterDELETE<&'a Self, CanBeResentFrom, i64> for ApplicationUserResetPasswordToken_6 {
    fn get(subject: &'a Self) -> i64 {
        return subject.can_be_resent_from;
    }
}