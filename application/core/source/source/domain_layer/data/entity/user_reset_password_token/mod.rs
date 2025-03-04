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
};
pub struct UserResetPasswordToken {
    user__id: User_Id,
    user_device__id: UserDevice_Id,
    value: Value,
    wrong_enter_tries_quantity: WrongEnterTriesQuantity,
    is_approved: IsApproved,
    expires_at: ExpiresAt,
    can_be_resent_from: CanBeResentFrom,
}
pub type UserResetPasswordToken_CanBeResentFrom = CanBeResentFrom;
pub type UserResetPasswordToken_ExpiresAt = ExpiresAt;
pub type UserResetPasswordToken_Value = Value;
pub type UserResetPasswordToken_WrongEnterTriesQuantity = WrongEnterTriesQuantity;
