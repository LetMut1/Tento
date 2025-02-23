mod field;
use {
    self::field::{
        CanBeResentFrom,
        ExpiresAt,
        IsApproved,
        Value,
        WrongEnterTriesQuantity,
    },
    super::{
        user::User_Id,
        user_device::UserDevice_Id,
    },
    std::marker::PhantomData,
};
pub struct UserResetPasswordToken {
    user__id: i64,
    _user__id: PhantomData<User_Id>,
    user_device__id: String,
    _user_device__id: PhantomData<UserDevice_Id>,
    value: String,
    _value: PhantomData<Value>,
    wrong_enter_tries_quantity: i16,
    _wrong_enter_tries_quantity: PhantomData<WrongEnterTriesQuantity>,
    is_approved: bool,
    _is_approved: PhantomData<IsApproved>,
    expires_at: i64,
    _expires_at: PhantomData<ExpiresAt>,
    can_be_resent_from: i64,
    _can_be_resent_from: PhantomData<CanBeResentFrom>,
}
pub type UserResetPasswordToken_CanBeResentFrom = CanBeResentFrom;
pub type UserResetPasswordToken_ExpiresAt = ExpiresAt;
pub type UserResetPasswordToken_IsApproved = IsApproved;
pub type UserResetPasswordToken_Value = Value;
pub type UserResetPasswordToken_WrongEnterTriesQuantity = WrongEnterTriesQuantity;
