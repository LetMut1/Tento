use crate::domain_layer::functionality::service::getter::Getter;
use extern_crate::serde::Serialize;
use extern_crate::serde::Deserialize;
use std::borrow::Cow;
use super::application_user_device::ApplicationUserDevice_Id;
use super::application_user::ApplicationUser_Email;

pub use self::Value as ApplicationUserRegistrationToken_Value;
pub use self::WrongEnterTriesQuantity as ApplicationUserRegistrationToken_WrongEnterTriesQuantity;
pub use self::IsApproved as ApplicationUserRegistrationToken_IsApproved;
pub use self::ExpiresAt as ApplicationUserRegistrationToken_ExpiresAt;
pub use self::CanBeResentFrom as ApplicationUserRegistrationToken_CanBeResentFrom;

#[cfg_attr(feature = "facilitate_non_automatic_functional_testing", derive(Serialize))]
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
#[cfg_attr(feature = "facilitate_non_automatic_functional_testing", derive(Deserialize))]
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
    application_user_email: Cow<'a, ApplicationUser_Email>,
    application_user_device_id: Cow<'a, ApplicationUserDevice_Id>,
    value: Value,
    wrong_enter_tries_quantity: WrongEnterTriesQuantity,
    is_approved: IsApproved,
    expires_at: ExpiresAt,
    can_be_resent_from: CanBeResentFrom
}

impl<'a> ApplicationUserRegistrationToken<'a> {
    pub const QUANTITY_OF_MINUTES_BEFORE_RESENDING: i64 = 1;
    pub const QUANTITY_OF_MINUTES_FOR_EXPIRATION: i64 = 60 * 3;
    pub const WRONG_ENTER_TRIES_QUANTITY_LIMIT: i16 = 5;

    pub fn new(
        application_user_email: Cow<'a, ApplicationUser_Email>,
        application_user_device_id: Cow<'a, ApplicationUserDevice_Id>,
        value: Value,
        wrong_enter_tries_quantity: WrongEnterTriesQuantity,
        is_approved: IsApproved,
        expires_at: ExpiresAt,
        can_be_resent_from: CanBeResentFrom
    ) -> Self {
        return Self {
            application_user_email,
            application_user_device_id,
            value,
            wrong_enter_tries_quantity,
            is_approved,
            expires_at,
            can_be_resent_from
        };
    }

    pub fn get_application_user_email<'b>(&'b self) -> &'b ApplicationUser_Email {
        return self.application_user_email.as_ref();
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

    pub fn get_is_approved<'b>(&'b self) -> IsApproved {
        return self.is_approved;
    }

    pub fn get_expires_at<'b>(&'b self) -> ExpiresAt {
        return self.expires_at;
    }

    pub fn get_can_be_resent_from<'b>(&'b self) -> CanBeResentFrom {
        return self.can_be_resent_from;
    }

    pub fn set_value<'b>(&'b mut self, value: Value) -> &'b mut Self {
        self.value = value;

        return self;
    }

    pub fn set_wrong_enter_tries_quantity<'b>(
        &'b mut self,
        wrong_enter_tries_quantity: WrongEnterTriesQuantity
    ) -> &'b mut Self {
        self.wrong_enter_tries_quantity = wrong_enter_tries_quantity;

        return self;
    }

    pub fn set_is_approved<'b>(&'b mut self, is_approved: IsApproved) -> &'b mut Self {
        self.is_approved = is_approved;

        return self;
    }

    pub fn set_expires_at<'b>(&'b mut self, expires_at: ExpiresAt) -> &'b mut Self {
        self.expires_at = expires_at;

        return self;
    }

    pub fn set_can_be_resent_from<'b>(&'b mut self, can_be_resent_from: CanBeResentFrom) -> &'b mut Self {
        self.can_be_resent_from = can_be_resent_from;

        return self;
    }
}

impl<'a> Getter<'a, &'a ApplicationUser_Email> for ApplicationUserRegistrationToken<'_> {
    fn get(&'a self) -> &'a ApplicationUser_Email {
        return self.get_application_user_email();
    }
}

impl<'a> Getter<'a, &'a ApplicationUserDevice_Id> for ApplicationUserRegistrationToken<'_> {
    fn get(&'a self) -> &'a ApplicationUserDevice_Id {
        return self.get_application_user_device_id();
    }
}

impl<'a> Getter<'a, &'a Value> for ApplicationUserRegistrationToken<'_> {
    fn get(&'a self) -> &'a Value {
        return self.get_value();
    }
}

impl<'a> Getter<'a, WrongEnterTriesQuantity> for ApplicationUserRegistrationToken<'_> {
    fn get(&'a self) -> WrongEnterTriesQuantity {
        return self.get_wrong_enter_tries_quantity();
    }
}

impl<'a> Getter<'a, IsApproved> for ApplicationUserRegistrationToken<'_> {
    fn get(&'a self) -> IsApproved {
        return self.get_is_approved();
    }
}

impl<'a> Getter<'a, ExpiresAt> for ApplicationUserRegistrationToken<'_> {
    fn get(&'a self) -> ExpiresAt {
        return self.get_expires_at();
    }
}

impl<'a> Getter<'a, CanBeResentFrom> for ApplicationUserRegistrationToken<'_> {
    fn get(&'a self) -> CanBeResentFrom {
        return self.get_can_be_resent_from();
    }
}

pub struct ApplicationUserRegistrationToken_1 {
    value: Value,
    wrong_enter_tries_quantity: WrongEnterTriesQuantity,
    is_approved: IsApproved,
    expires_at: ExpiresAt,
    can_be_resent_from: CanBeResentFrom
}

impl ApplicationUserRegistrationToken_1 {
    pub fn new(
        value: Value,
        wrong_enter_tries_quantity: WrongEnterTriesQuantity,
        is_approved: IsApproved,
        expires_at: ExpiresAt,
        can_be_resent_from: CanBeResentFrom
    ) -> Self {
        return Self {
            value,
            wrong_enter_tries_quantity,
            is_approved,
            expires_at,
            can_be_resent_from
        };
    }

    pub fn get_value<'b>(&'b self) -> &'b Value {
        return &self.value;
    }

    pub fn get_wrong_enter_tries_quantity<'b>(&'b self) -> WrongEnterTriesQuantity {
        return self.wrong_enter_tries_quantity;
    }

    pub fn get_is_approved<'b>(&'b self) -> IsApproved {
        return self.is_approved;
    }

    pub fn get_expires_at<'b>(&'b self) -> ExpiresAt {
        return self.expires_at;
    }

    pub fn get_can_be_resent_from<'b>(&'b self) -> CanBeResentFrom {
        return self.can_be_resent_from;
    }

    pub fn set_value<'b>(&'b mut self, value: Value) -> &'b mut Self {
        self.value = value;

        return self;
    }

    pub fn set_wrong_enter_tries_quantity<'b>(
        &'b mut self,
        wrong_enter_tries_quantity: WrongEnterTriesQuantity
    ) -> &'b mut Self {
        self.wrong_enter_tries_quantity = wrong_enter_tries_quantity;

        return self;
    }

    pub fn set_is_approved<'b>(&'b mut self, is_approved: IsApproved) -> &'b mut Self {
        self.is_approved = is_approved;

        return self;
    }

    pub fn set_expires_at<'b>(&'b mut self, expires_at: ExpiresAt) -> &'b mut Self {
        self.expires_at = expires_at;

        return self;
    }

    pub fn set_can_be_resent_from<'b>(&'b mut self, can_be_resent_from: CanBeResentFrom) -> &'b mut Self {
        self.can_be_resent_from = can_be_resent_from;

        return self;
    }
}

impl<'a> Getter<'a, &'a Value> for ApplicationUserRegistrationToken_1 {
    fn get(&'a self) -> &'a Value {
        return self.get_value();
    }
}

impl<'a> Getter<'a, WrongEnterTriesQuantity> for ApplicationUserRegistrationToken_1 {
    fn get(&'a self) -> WrongEnterTriesQuantity {
        return self.get_wrong_enter_tries_quantity();
    }
}

impl<'a> Getter<'a, IsApproved> for ApplicationUserRegistrationToken_1 {
    fn get(&'a self) -> IsApproved {
        return self.get_is_approved();
    }
}

impl<'a> Getter<'a, ExpiresAt> for ApplicationUserRegistrationToken_1 {
    fn get(&'a self) -> ExpiresAt {
        return self.get_expires_at();
    }
}

impl<'a> Getter<'a, CanBeResentFrom> for ApplicationUserRegistrationToken_1 {
    fn get(&'a self) -> CanBeResentFrom {
        return self.get_can_be_resent_from();
    }
}

pub struct ApplicationUserRegistrationToken_2 {
    can_be_resent_from: CanBeResentFrom
}

pub struct ApplicationUserRegistrationToken_3 {
    value: Value,
    wrong_enter_tries_quantity: WrongEnterTriesQuantity,
    is_approved: IsApproved,
    expires_at: ExpiresAt
}

impl ApplicationUserRegistrationToken_3 {
    pub fn new(
        value: Value,
        wrong_enter_tries_quantity: WrongEnterTriesQuantity,
        is_approved: IsApproved,
        expires_at: ExpiresAt
    ) -> Self {
        return Self {
            value,
            wrong_enter_tries_quantity,
            is_approved,
            expires_at
        };
    }

    pub fn get_value<'b>(&'b self) -> &'b Value {
        return &self.value;
    }

    pub fn get_wrong_enter_tries_quantity<'b>(&'b self) -> WrongEnterTriesQuantity {
        return self.wrong_enter_tries_quantity;
    }

    pub fn get_is_approved<'b>(&'b self) -> IsApproved {
        return self.is_approved;
    }

    pub fn get_expires_at<'b>(&'b self) -> ExpiresAt {
        return self.expires_at;
    }

    pub fn set_value<'b>(&'b mut self, value: Value) -> &'b mut Self {
        self.value = value;

        return self;
    }

    pub fn set_wrong_enter_tries_quantity<'b>(
        &'b mut self,
        wrong_enter_tries_quantity: WrongEnterTriesQuantity
    ) -> &'b mut Self {
        self.wrong_enter_tries_quantity = wrong_enter_tries_quantity;

        return self;
    }

    pub fn set_is_approved<'b>(&'b mut self, is_approved: IsApproved) -> &'b mut Self {
        self.is_approved = is_approved;

        return self;
    }

    pub fn set_expires_at<'b>(&'b mut self, expires_at: ExpiresAt) -> &'b mut Self {
        self.expires_at = expires_at;

        return self;
    }
}

impl<'a> Getter<'a, &'a Value> for ApplicationUserRegistrationToken_3 {
    fn get(&'a self) -> &'a Value {
        return self.get_value();
    }
}

impl<'a> Getter<'a, WrongEnterTriesQuantity> for ApplicationUserRegistrationToken_3 {
    fn get(&'a self) -> WrongEnterTriesQuantity {
        return self.get_wrong_enter_tries_quantity();
    }
}

impl<'a> Getter<'a, IsApproved> for ApplicationUserRegistrationToken_3 {
    fn get(&'a self) -> IsApproved {
        return self.get_is_approved();
    }
}

impl<'a> Getter<'a, ExpiresAt> for ApplicationUserRegistrationToken_3 {
    fn get(&'a self) -> ExpiresAt {
        return self.get_expires_at();
    }
}

pub struct ApplicationUserRegistrationToken_4 {
    wrong_enter_tries_quantity: WrongEnterTriesQuantity
}

pub struct ApplicationUserRegistrationToken_5 {
    is_approved: IsApproved
}

pub struct ApplicationUserRegistrationToken_6 {
    value: Value,
    is_approved: IsApproved,
    expires_at: ExpiresAt,
    can_be_resent_from: CanBeResentFrom
}

impl ApplicationUserRegistrationToken_6 {
    pub fn new(
        value: Value,
        is_approved: IsApproved,
        expires_at: ExpiresAt,
        can_be_resent_from: CanBeResentFrom
    ) -> Self {
        return Self {
            value,
            is_approved,
            expires_at,
            can_be_resent_from
        };
    }

    pub fn get_value<'b>(&'b self) -> &'b Value {
        return &self.value;
    }

    pub fn get_is_approved<'b>(&'b self) -> IsApproved {
        return self.is_approved;
    }

    pub fn get_expires_at<'b>(&'b self) -> ExpiresAt {
        return self.expires_at;
    }

    pub fn get_can_be_resent_from<'b>(&'b self) -> CanBeResentFrom {
        return self.can_be_resent_from;
    }

    pub fn set_can_be_resent_from<'b>(&'b mut self, can_be_resent_from: CanBeResentFrom) -> &'b mut Self {
        self.can_be_resent_from = can_be_resent_from;

        return self;
    }
}

impl<'a> Getter<'a, &'a Value> for ApplicationUserRegistrationToken_6 {
    fn get(&'a self) -> &'a Value {
        return self.get_value();
    }
}

impl<'a> Getter<'a, IsApproved> for ApplicationUserRegistrationToken_6 {
    fn get(&'a self) -> IsApproved {
        return self.get_is_approved();
    }
}

impl<'a> Getter<'a, ExpiresAt> for ApplicationUserRegistrationToken_6 {
    fn get(&'a self) -> ExpiresAt {
        return self.get_expires_at();
    }
}

impl<'a> Getter<'a, CanBeResentFrom> for ApplicationUserRegistrationToken_6 {
    fn get(&'a self) -> CanBeResentFrom {
        return self.get_can_be_resent_from();
    }
}