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
        user::User_Email,
        user_device::UserDevice_Id,
    },
    std::marker::PhantomData,
};
pub struct UserRegistrationToken {
    user__email: PhantomData<(String, User_Email)>,
    user_device__id: PhantomData<(String, UserDevice_Id)>,
    value: PhantomData<(String, Value)>,
    wrong_enter_tries_quantity: PhantomData<(i16, WrongEnterTriesQuantity)>,
    is_approved: PhantomData<(bool, IsApproved)>,
    expires_at: PhantomData<(i64, ExpiresAt)>,
    can_be_resent_from: PhantomData<(i64, CanBeResentFrom)>,
}
pub type UserRegistrationToken_CanBeResentFrom = CanBeResentFrom;
pub type UserRegistrationToken_ExpiresAt = ExpiresAt;
pub type UserRegistrationToken_IsApproved = IsApproved;
pub type UserRegistrationToken_Value = Value;
pub type UserRegistrationToken_WrongEnterTriesQuantity = WrongEnterTriesQuantity;
