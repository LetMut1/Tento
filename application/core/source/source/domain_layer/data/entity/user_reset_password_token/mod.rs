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
    user__id: PhantomData<(i64, User_Id)>,
    user_device__id: PhantomData<(String, UserDevice_Id)>,
    value: PhantomData<(String, Value)>,
    wrong_enter_tries_quantity: PhantomData<(i16, WrongEnterTriesQuantity)>,
    is_approved: PhantomData<(bool, IsApproved)>,
    expires_at: PhantomData<(i64, ExpiresAt)>,
    can_be_resent_from: PhantomData<(i64, CanBeResentFrom)>,
}
pub type UserResetPasswordToken_CanBeResentFrom = CanBeResentFrom;
pub type UserResetPasswordToken_ExpiresAt = ExpiresAt;
pub type UserResetPasswordToken_IsApproved = IsApproved;
pub type UserResetPasswordToken_Value = Value;
pub type UserResetPasswordToken_WrongEnterTriesQuantity = WrongEnterTriesQuantity;
