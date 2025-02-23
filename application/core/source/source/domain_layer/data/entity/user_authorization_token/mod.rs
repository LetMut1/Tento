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
    user__id: i64,
    _user__id: PhantomData<User_Id>,
    user_device__id: String,
    _user_device__id: PhantomData<UserDevice_Id>,
    value: String,
    _value: PhantomData<Value>,
    wrong_enter_tries_quantity: i16,
    _wrong_enter_tries_quantity: PhantomData<WrongEnterTriesQuantity>,
    expires_at: i64,
    _expires_at: PhantomData<ExpiresAt>,
    can_be_resent_from: i64,
    _can_be_resent_from: PhantomData<CanBeResentFrom>,
}
pub type UserAuthorizationToken_CanBeResentFrom = CanBeResentFrom;
pub type UserAuthorizationToken_ExpiresAt = ExpiresAt;
pub type UserAuthorizationToken_Value = Value;
pub type UserAuthorizationToken_WrongEnterTriesQuantity = WrongEnterTriesQuantity;
