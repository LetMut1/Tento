pub mod derivative;
mod field;
use self::field::{
    CanBeResentFrom,
    ExpiresAt,
    IsApproved,
    Value,
    WrongEnterTriesQuantity,
};
use super::{
    user::User_Id,
    user_device::UserDevice_Id,
};
use std::{
    borrow::Cow,
    marker::PhantomData,
};
pub struct UserResetPasswordToken<'a> {
    pub application_user__id: i64,
    _application_user__id: PhantomData<User_Id>,
    pub application_user_device__id: Cow<'a, str>,
    _application_user_device__id: PhantomData<UserDevice_Id>,
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
impl<'a> UserResetPasswordToken<'a> {
    pub fn new(
        application_user__id: i64,
        application_user_device__id: Cow<'a, str>,
        value: String,
        wrong_enter_tries_quantity: i16,
        is_approved: bool,
        expires_at: i64,
        can_be_resent_from: i64,
    ) -> Self {
        return Self {
            application_user__id,
            _application_user__id: PhantomData,
            application_user_device__id,
            _application_user_device__id: PhantomData,
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
pub type UserResetPasswordToken_CanBeResentFrom = CanBeResentFrom;
pub type UserResetPasswordToken_ExpiresAt = ExpiresAt;
pub type UserResetPasswordToken_IsApproved = IsApproved;
pub type UserResetPasswordToken_Value = Value;
pub type UserResetPasswordToken_WrongEnterTriesQuantity = WrongEnterTriesQuantity;