mod derivative;
pub use self::derivative::{
    UserRegistrationToken_1,
    UserRegistrationToken_2,
    UserRegistrationToken_3,
};
mod field;
use self::field::{
    CanBeResentFrom,
    ExpiresAt,
    IsApproved,
    Value,
    WrongEnterTriesQuantity,
};
use super::{
    user::User_Email,
    user_device::UserDevice_Id,
};
use std::{
    borrow::Cow,
    marker::PhantomData,
};
pub struct UserRegistrationToken<'a> {
    pub user__email: Cow<'a, str>,
    _user__email: PhantomData<User_Email>,
    pub user_device__id: Cow<'a, str>,
    _user_device__id: PhantomData<UserDevice_Id>,
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
impl<'a> UserRegistrationToken<'a> {
    pub fn new(
        user__email: Cow<'a, str>,
        user_device__id: Cow<'a, str>,
        value: String,
        wrong_enter_tries_quantity: i16,
        is_approved: bool,
        expires_at: i64,
        can_be_resent_from: i64,
    ) -> Self {
        return Self {
            user__email,
            _user__email: PhantomData,
            user_device__id,
            _user_device__id: PhantomData,
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
pub type UserRegistrationToken_CanBeResentFrom = CanBeResentFrom;
pub type UserRegistrationToken_ExpiresAt = ExpiresAt;
pub type UserRegistrationToken_IsApproved = IsApproved;
pub type UserRegistrationToken_Value = Value;
pub type UserRegistrationToken_WrongEnterTriesQuantity = WrongEnterTriesQuantity;
