use crate::domain_layer::functionality::service::getter::Getter;
use std::borrow::Cow;
use std::marker::PhantomData;
use super::application_user_device::ApplicationUserDevice_Id;
use super::application_user::ApplicationUser_Id;

pub use self::Value as ApplicationUserAuthorizationToken_Value;
pub use self::WrongEnterTriesQuantity as ApplicationUserAuthorizationToken_WrongEnterTriesQuantity;
pub use self::ExpiresAt as ApplicationUserAuthorizationToken_ExpiresAt;
pub use self::CanBeResentFrom as ApplicationUserAuthorizationToken_CanBeResentFrom;

pub struct ApplicationUserAuthorizationToken<'a> {
    application_user_id: i64,
    _application_user_id: PhantomData<ApplicationUser_Id>,

    application_user_device_id: Cow<'a, str>,
    _application_user_device_id: PhantomData<ApplicationUserDevice_Id>,

    value: String,
    _value: PhantomData<Value>,

    wrong_enter_tries_quantity: i16,
    _wrong_enter_tries_quantity: PhantomData<WrongEnterTriesQuantity>,

    expires_at: i64,
    _expires_at: PhantomData<ExpiresAt>,

    can_be_resent_from: i64,
    _can_be_resent_from: PhantomData<CanBeResentFrom>,
}

impl<'a> ApplicationUserAuthorizationToken<'a> {
    pub const QUANTITY_OF_MINUTES_BEFORE_RESENDING: i64 = 1;
    pub const QUANTITY_OF_MINUTES_FOR_EXPIRATION: i64 = 10;
    pub const WRONG_ENTER_TRIES_QUANTITY_LIMIT: i16 = 5;

    pub fn new(
        application_user_id: i64,
        application_user_device_id: Cow<'a, str>,
        value: String,
        wrong_enter_tries_quantity: i16,
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

    pub fn set_expires_at<'b>(&'b mut self, expires_at: i64) -> &'b mut Self {
        self.expires_at = expires_at;

        return self;
    }

    pub fn set_can_be_resent_from<'b>(&'b mut self, can_be_resent_from: i64) -> &'b mut Self {
        self.can_be_resent_from = can_be_resent_from;

        return self;
    }
}

impl<'a> Getter<&'a Self, ApplicationUser_Id, i64> for ApplicationUserAuthorizationToken<'_> {
    fn get(subject: &'a Self) -> i64 {
        return subject.application_user_id;
    }
}

impl<'a, 'b: 'a> Getter<&'a Self, ApplicationUserDevice_Id, &'a str> for ApplicationUserAuthorizationToken<'b> {
    fn get(subject: &'a Self) -> &'a str {
        return subject.application_user_device_id.as_ref();
    }
}

impl<'a> Getter<&'a Self, Value, &'a str> for ApplicationUserAuthorizationToken<'_> {
    fn get(subject: &'a Self) -> &'a str {
        return subject.value.as_str();
    }
}

impl<'a> Getter<&'a Self, WrongEnterTriesQuantity, i16> for ApplicationUserAuthorizationToken<'_> {
    fn get(subject: &'a Self) -> i16 {
        return subject.wrong_enter_tries_quantity;
    }
}

impl<'a> Getter<&'a Self, ExpiresAt, i64> for ApplicationUserAuthorizationToken<'_> {
    fn get(subject: &'a Self) -> i64 {
        return subject.expires_at;
    }
}

impl<'a> Getter<&'a Self, CanBeResentFrom, i64> for ApplicationUserAuthorizationToken<'_> {
    fn get(subject: &'a Self) -> i64 {
        return subject.can_be_resent_from;
    }
}

pub struct ApplicationUserAuthorizationToken_1 {
    value: String,
    _value: PhantomData<Value>,

    wrong_enter_tries_quantity: i16,
    _wrong_enter_tries_quantity: PhantomData<WrongEnterTriesQuantity>,

    expires_at: i64,
    _expires_at: PhantomData<ExpiresAt>,

    can_be_resent_from: i64,
    _can_be_resent_from: PhantomData<CanBeResentFrom>,
}

impl ApplicationUserAuthorizationToken_1 {
    pub fn new(
        value: String,
        wrong_enter_tries_quantity: i16,
        expires_at: i64,
        can_be_resent_from: i64
    ) -> Self {
        return Self {
            value,
            _value: PhantomData,
            wrong_enter_tries_quantity,
            _wrong_enter_tries_quantity: PhantomData,
            expires_at,
            _expires_at: PhantomData,
            can_be_resent_from,
            _can_be_resent_from: PhantomData
        };
    }

    pub fn get_value<'a>(&'a self) -> &'a str {
        return self.value.as_str();
    }

    pub fn get_wrong_enter_tries_quantity<'a>(&'a self) -> i16 {
        return self.wrong_enter_tries_quantity;
    }

    pub fn get_expires_at<'a>(&'a self) -> i64 {
        return self.expires_at;
    }

    pub fn get_can_be_resent_from<'a>(&'a self) -> i64 {
        return self.can_be_resent_from;
    }

    pub fn set_value<'a>(&'a mut self, value: String) -> &'a mut Self {
        self.value = value;

        return self;
    }

    pub fn set_wrong_enter_tries_quantity<'a>(&'a mut self, wrong_enter_tries_quantity: i16) -> &'a mut Self {
        self.wrong_enter_tries_quantity = wrong_enter_tries_quantity;

        return self;
    }

    pub fn set_expires_at<'a>(&'a mut self, expires_at: i64) -> &'a mut Self {
        self.expires_at = expires_at;

        return self;
    }

    pub fn set_can_be_resent_from<'a>(&'a mut self, can_be_resent_from: i64) -> &'a mut Self {
        self.can_be_resent_from = can_be_resent_from;

        return self;
    }
}

impl<'a> Getter<&'a Self, Value, &'a str> for ApplicationUserAuthorizationToken_1 {
    fn get(subject: &'a Self) -> &'a str {
        return subject.value.as_str();
    }
}

impl<'a> Getter<&'a Self, WrongEnterTriesQuantity, i16> for ApplicationUserAuthorizationToken_1 {
    fn get(subject: &'a Self) -> i16 {
        return subject.wrong_enter_tries_quantity;
    }
}

impl<'a> Getter<&'a Self, ExpiresAt, i64> for ApplicationUserAuthorizationToken_1 {
    fn get(subject: &'a Self) -> i64 {
        return subject.expires_at;
    }
}

impl<'a> Getter<&'a Self, CanBeResentFrom, i64> for ApplicationUserAuthorizationToken_1 {
    fn get(subject: &'a Self) -> i64 {
        return subject.can_be_resent_from;
    }
}

pub struct ApplicationUserAuthorizationToken_2 {
    value: String,
    _value: PhantomData<Value>,

    wrong_enter_tries_quantity: i16,
    _wrong_enter_tries_quantity: PhantomData<WrongEnterTriesQuantity>,

    expires_at: i64,
    _expires_at: PhantomData<ExpiresAt>
}

impl ApplicationUserAuthorizationToken_2 {
    pub fn new(
        value: String,
        wrong_enter_tries_quantity: i16,
        expires_at: i64
    ) -> Self {
        return Self {
            value,
            _value: PhantomData,
            wrong_enter_tries_quantity,
            _wrong_enter_tries_quantity: PhantomData,
            expires_at,
            _expires_at: PhantomData
        };
    }

    pub fn get_value<'a>(&'a self) -> &'a str {
        return self.value.as_str();
    }

    pub fn get_wrong_enter_tries_quantity<'a>(&'a self) -> i16 {
        return self.wrong_enter_tries_quantity;
    }

    pub fn get_expires_at<'a>(&'a self) -> i64 {
        return self.expires_at;
    }

    pub fn set_value<'a>(&'a mut self, value: String) -> &'a mut Self {
        self.value = value;

        return self;
    }

    pub fn set_wrong_enter_tries_quantity<'a>(&'a mut self, wrong_enter_tries_quantity: i16) -> &'a mut Self {
        self.wrong_enter_tries_quantity = wrong_enter_tries_quantity;

        return self;
    }

    pub fn set_expires_at<'a>(&'a mut self, expires_at: i64) -> &'a mut Self {
        self.expires_at = expires_at;

        return self;
    }
}

impl<'a> Getter<&'a Self, Value, &'a str> for ApplicationUserAuthorizationToken_2 {
    fn get(subject: &'a Self) -> &'a str {
        return subject.value.as_str();
    }
}

impl<'a> Getter<&'a Self, WrongEnterTriesQuantity, i16> for ApplicationUserAuthorizationToken_2 {
    fn get(subject: &'a Self) -> i16 {
        return subject.wrong_enter_tries_quantity;
    }
}

impl<'a> Getter<&'a Self, ExpiresAt, i64> for ApplicationUserAuthorizationToken_2 {
    fn get(subject: &'a Self) -> i64 {
        return subject.expires_at;
    }
}

pub struct ApplicationUserAuthorizationToken_3 {
    can_be_resent_from: i64,
    _can_be_resent_from: PhantomData<CanBeResentFrom>,
}

pub struct ApplicationUserAuthorizationToken_4 {
    wrong_enter_tries_quantity: i16,
    _wrong_enter_tries_quantity: PhantomData<WrongEnterTriesQuantity>
}

pub struct ApplicationUserAuthorizationToken_5 {
    expires_at: i64,
    _expires_at: PhantomData<ExpiresAt>,

    can_be_resent_from: i64,
    _can_be_resent_from: PhantomData<CanBeResentFrom>,
}

pub struct Value;

pub struct WrongEnterTriesQuantity;

pub struct ExpiresAt;

pub struct CanBeResentFrom;