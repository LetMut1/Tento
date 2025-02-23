mod field;
use {
    self::field::{
        CanBeResentFrom,
        ExpiresAt,
        Value,
        WrongEnterTriesQuantity,
    },
    super::{
        user::User_Id,
        user_device::UserDevice_Id,
    },
    std::marker::PhantomData,
};
pub struct UserAuthorizationToken {
    user__id: PhantomData<(i64, User_Id)>,
    user_device__id: PhantomData<(String, UserDevice_Id)>,
    value: PhantomData<(String, Value)>,
    wrong_enter_tries_quantity: PhantomData<(i16, WrongEnterTriesQuantity)>,
    expires_at: PhantomData<(i64, ExpiresAt)>,
    can_be_resent_from: PhantomData<(i64, CanBeResentFrom)>,
}
pub type UserAuthorizationToken_CanBeResentFrom = CanBeResentFrom;
pub type UserAuthorizationToken_ExpiresAt = ExpiresAt;
pub type UserAuthorizationToken_Value = Value;
pub type UserAuthorizationToken_WrongEnterTriesQuantity = WrongEnterTriesQuantity;
