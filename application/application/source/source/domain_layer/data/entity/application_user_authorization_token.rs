use super::application_user::ApplicationUser_Id;
use super::application_user_device::ApplicationUserDevice_Id;
use crate::domain_layer::functionality::service::getter::Getter;
use extern_crate::serde::Deserialize;
use extern_crate::serde::Serialize;
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
pub struct ExpiresAt(i64);

impl ExpiresAt {
    pub fn new(inner: i64) -> Self {
        return Self(inner);
    }

    pub fn get<'a>(&'a self) -> i64 {
        return self.0;
    }
}

#[cfg_attr(
    feature = "manual_testing",
    derive(Deserialize)
)]
#[derive(Clone, Copy, Serialize)]
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

pub struct ApplicationUserAuthorizationToken<'a> {
    application_user_id: ApplicationUser_Id,
    application_user_device_id: Cow<'a, ApplicationUserDevice_Id>,
    value: Value,
    wrong_enter_tries_quantity: WrongEnterTriesQuantity,
    expires_at: ExpiresAt,
    can_be_resent_from: CanBeResentFrom,
}

impl<'a> ApplicationUserAuthorizationToken<'a> {
    pub const QUANTITY_OF_MINUTES_BEFORE_RESENDING: i64 = 1;
    pub const QUANTITY_OF_MINUTES_FOR_EXPIRATION: i64 = 10;
    pub const WRONG_ENTER_TRIES_QUANTITY_LIMIT: i16 = 5;

    pub fn new(
        application_user_id: ApplicationUser_Id,
        application_user_device_id: Cow<'a, ApplicationUserDevice_Id>,
        value: Value,
        wrong_enter_tries_quantity: WrongEnterTriesQuantity,
        expires_at: ExpiresAt,
        can_be_resent_from: CanBeResentFrom,
    ) -> Self {
        return Self {
            application_user_id,
            application_user_device_id,
            value,
            wrong_enter_tries_quantity,
            expires_at,
            can_be_resent_from,
        };
    }

    pub fn get_application_user_id<'b>(&'b self) -> ApplicationUser_Id {
        return self.application_user_id;
    }

    pub fn get_application_user_device_id<'b>(&'b self) -> &'b ApplicationUserDevice_Id {
        return self.application_user_device_id.as_ref();
    }

    pub fn get_value<'b>(&'b self) -> &'b Value {
        return &self.value;
    }

    pub fn get_wrong_enter_tries_quantity<'b>(&'b self) -> WrongEnterTriesQuantity {
        return self.wrong_enter_tries_quantity;
    }

    pub fn get_expires_at<'b>(&'b self) -> ExpiresAt {
        return self.expires_at;
    }

    pub fn get_can_be_resent_from<'b>(&'b self) -> CanBeResentFrom {
        return self.can_be_resent_from;
    }

    pub fn set_value<'b>(
        &'b mut self,
        value: Value,
    ) -> &'b mut Self {
        self.value = value;

        return self;
    }

    pub fn set_wrong_enter_tries_quantity<'b>(
        &'b mut self,
        wrong_enter_tries_quantity: WrongEnterTriesQuantity,
    ) -> &'b mut Self {
        self.wrong_enter_tries_quantity = wrong_enter_tries_quantity;

        return self;
    }

    pub fn set_expires_at<'b>(
        &'b mut self,
        expires_at: ExpiresAt,
    ) -> &'b mut Self {
        self.expires_at = expires_at;

        return self;
    }

    pub fn set_can_be_resent_from<'b>(
        &'b mut self,
        can_be_resent_from: CanBeResentFrom,
    ) -> &'b mut Self {
        self.can_be_resent_from = can_be_resent_from;

        return self;
    }
}

impl<'a> Getter<'a, ApplicationUser_Id> for ApplicationUserAuthorizationToken<'_> {
    fn get(&'a self) -> ApplicationUser_Id {
        return self.get_application_user_id();
    }
}

impl<'a> Getter<'a, &'a ApplicationUserDevice_Id> for ApplicationUserAuthorizationToken<'_> {
    fn get(&'a self) -> &'a ApplicationUserDevice_Id {
        return self.get_application_user_device_id();
    }
}

impl<'a> Getter<'a, &'a Value> for ApplicationUserAuthorizationToken<'_> {
    fn get(&'a self) -> &'a Value {
        return self.get_value();
    }
}

impl<'a> Getter<'a, WrongEnterTriesQuantity> for ApplicationUserAuthorizationToken<'_> {
    fn get(&'a self) -> WrongEnterTriesQuantity {
        return self.get_wrong_enter_tries_quantity();
    }
}

impl<'a> Getter<'a, ExpiresAt> for ApplicationUserAuthorizationToken<'_> {
    fn get(&'a self) -> ExpiresAt {
        return self.get_expires_at();
    }
}

impl<'a> Getter<'a, CanBeResentFrom> for ApplicationUserAuthorizationToken<'_> {
    fn get(&'a self) -> CanBeResentFrom {
        return self.get_can_be_resent_from();
    }
}

pub struct ApplicationUserAuthorizationToken1 {
    value: Value,
    wrong_enter_tries_quantity: WrongEnterTriesQuantity,
    expires_at: ExpiresAt,
    can_be_resent_from: CanBeResentFrom,
}

impl ApplicationUserAuthorizationToken1 {
    pub fn new(
        value: Value,
        wrong_enter_tries_quantity: WrongEnterTriesQuantity,
        expires_at: ExpiresAt,
        can_be_resent_from: CanBeResentFrom,
    ) -> Self {
        return Self {
            value,
            wrong_enter_tries_quantity,
            expires_at,
            can_be_resent_from,
        };
    }

    pub fn get_value<'a>(&'a self) -> &'a Value {
        return &self.value;
    }

    pub fn get_wrong_enter_tries_quantity<'a>(&'a self) -> WrongEnterTriesQuantity {
        return self.wrong_enter_tries_quantity;
    }

    pub fn get_expires_at<'a>(&'a self) -> ExpiresAt {
        return self.expires_at;
    }

    pub fn get_can_be_resent_from<'a>(&'a self) -> CanBeResentFrom {
        return self.can_be_resent_from;
    }

    pub fn set_value<'a>(
        &'a mut self,
        value: Value,
    ) -> &'a mut Self {
        self.value = value;

        return self;
    }

    pub fn set_wrong_enter_tries_quantity<'a>(
        &'a mut self,
        wrong_enter_tries_quantity: WrongEnterTriesQuantity,
    ) -> &'a mut Self {
        self.wrong_enter_tries_quantity = wrong_enter_tries_quantity;

        return self;
    }

    pub fn set_expires_at<'a>(
        &'a mut self,
        expires_at: ExpiresAt,
    ) -> &'a mut Self {
        self.expires_at = expires_at;

        return self;
    }

    pub fn set_can_be_resent_from<'a>(
        &'a mut self,
        can_be_resent_from: CanBeResentFrom,
    ) -> &'a mut Self {
        self.can_be_resent_from = can_be_resent_from;

        return self;
    }
}

impl<'a> Getter<'a, &'a Value> for ApplicationUserAuthorizationToken1 {
    fn get(&'a self) -> &'a Value {
        return self.get_value();
    }
}

impl<'a> Getter<'a, WrongEnterTriesQuantity> for ApplicationUserAuthorizationToken1 {
    fn get(&'a self) -> WrongEnterTriesQuantity {
        return self.get_wrong_enter_tries_quantity();
    }
}

impl<'a> Getter<'a, ExpiresAt> for ApplicationUserAuthorizationToken1 {
    fn get(&'a self) -> ExpiresAt {
        return self.get_expires_at();
    }
}

impl<'a> Getter<'a, CanBeResentFrom> for ApplicationUserAuthorizationToken1 {
    fn get(&'a self) -> CanBeResentFrom {
        return self.get_can_be_resent_from();
    }
}

pub struct ApplicationUserAuthorizationToken2 {
    value: Value,
    wrong_enter_tries_quantity: WrongEnterTriesQuantity,
    expires_at: ExpiresAt,
}

impl ApplicationUserAuthorizationToken2 {
    pub fn new(
        value: Value,
        wrong_enter_tries_quantity: WrongEnterTriesQuantity,
        expires_at: ExpiresAt,
    ) -> Self {
        return Self {
            value,
            wrong_enter_tries_quantity,
            expires_at,
        };
    }

    pub fn get_value<'a>(&'a self) -> &'a Value {
        return &self.value;
    }

    pub fn get_wrong_enter_tries_quantity<'a>(&'a self) -> WrongEnterTriesQuantity {
        return self.wrong_enter_tries_quantity;
    }

    pub fn get_wrong_enter_tries_quantity_<'a>(&'a mut self) -> &'a mut WrongEnterTriesQuantity {
        return &mut self.wrong_enter_tries_quantity;
    }

    pub fn get_expires_at<'a>(&'a self) -> ExpiresAt {
        return self.expires_at;
    }

    pub fn set_value<'a>(
        &'a mut self,
        value: Value,
    ) -> &'a mut Self {
        self.value = value;

        return self;
    }

    pub fn set_wrong_enter_tries_quantity<'a>(
        &'a mut self,
        wrong_enter_tries_quantity: WrongEnterTriesQuantity,
    ) -> &'a mut Self {
        self.wrong_enter_tries_quantity = wrong_enter_tries_quantity;

        return self;
    }

    pub fn set_expires_at<'a>(
        &'a mut self,
        expires_at: ExpiresAt,
    ) -> &'a mut Self {
        self.expires_at = expires_at;

        return self;
    }
}

impl<'a> Getter<'a, &'a Value> for ApplicationUserAuthorizationToken2 {
    fn get(&'a self) -> &'a Value {
        return self.get_value();
    }
}

impl<'a> Getter<'a, WrongEnterTriesQuantity> for ApplicationUserAuthorizationToken2 {
    fn get(&'a self) -> WrongEnterTriesQuantity {
        return self.get_wrong_enter_tries_quantity();
    }
}

impl<'a> Getter<'a, ExpiresAt> for ApplicationUserAuthorizationToken2 {
    fn get(&'a self) -> ExpiresAt {
        return self.expires_at;
    }
}

pub struct ApplicationUserAuthorizationToken3 {
    can_be_resent_from: CanBeResentFrom,
}

pub struct ApplicationUserAuthorizationToken4 {
    wrong_enter_tries_quantity: WrongEnterTriesQuantity,
}

pub struct ApplicationUserAuthorizationToken5 {
    value: Value,
    expires_at: ExpiresAt,
    can_be_resent_from: CanBeResentFrom,
}

impl ApplicationUserAuthorizationToken5 {
    pub fn new(
        value: Value,
        expires_at: ExpiresAt,
        can_be_resent_from: CanBeResentFrom,
    ) -> Self {
        return Self {
            value,
            expires_at,
            can_be_resent_from,
        };
    }

    pub fn get_value<'a>(&'a self) -> &'a Value {
        return &self.value;
    }

    pub fn get_expires_at<'a>(&'a self) -> ExpiresAt {
        return self.expires_at;
    }

    pub fn get_can_be_resent_from<'a>(&'a self) -> CanBeResentFrom {
        return self.can_be_resent_from;
    }

    pub fn set_expires_at<'a>(
        &'a mut self,
        expires_at: ExpiresAt,
    ) -> &'a mut Self {
        self.expires_at = expires_at;

        return self;
    }

    pub fn set_can_be_resent_from<'a>(
        &'a mut self,
        can_be_resent_from: CanBeResentFrom,
    ) -> &'a mut Self {
        self.can_be_resent_from = can_be_resent_from;

        return self;
    }
}

impl<'a> Getter<'a, &'a Value> for ApplicationUserAuthorizationToken5 {
    fn get(&'a self) -> &'a Value {
        return self.get_value();
    }
}

impl<'a> Getter<'a, ExpiresAt> for ApplicationUserAuthorizationToken5 {
    fn get(&'a self) -> ExpiresAt {
        return self.get_expires_at();
    }
}

impl<'a> Getter<'a, CanBeResentFrom> for ApplicationUserAuthorizationToken5 {
    fn get(&'a self) -> CanBeResentFrom {
        return self.get_can_be_resent_from();
    }
}
