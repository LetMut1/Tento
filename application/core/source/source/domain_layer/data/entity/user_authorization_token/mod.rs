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
};
pub struct UserAuthorizationToken {
    user__id: User_Id,
    user_device__id: UserDevice_Id,
    value: Value,
    wrong_enter_tries_quantity: WrongEnterTriesQuantity,
    expires_at: ExpiresAt,
    can_be_resent_from: CanBeResentFrom,
}
pub type UserAuthorizationToken_CanBeResentFrom = CanBeResentFrom;
pub type UserAuthorizationToken_ExpiresAt = ExpiresAt;
pub type UserAuthorizationToken_Value = Value;
pub type UserAuthorizationToken_WrongEnterTriesQuantity = WrongEnterTriesQuantity;
