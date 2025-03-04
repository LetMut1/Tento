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
};
pub struct UserRegistrationToken {
    user__email: User_Email,
    user_device__id: UserDevice_Id,
    value: Value,
    wrong_enter_tries_quantity: WrongEnterTriesQuantity,
    is_approved: IsApproved,
    expires_at: ExpiresAt,
    can_be_resent_from: CanBeResentFrom,
}
pub type UserRegistrationToken_CanBeResentFrom = CanBeResentFrom;
pub type UserRegistrationToken_ExpiresAt = ExpiresAt;
pub type UserRegistrationToken_Value = Value;
pub type UserRegistrationToken_WrongEnterTriesQuantity = WrongEnterTriesQuantity;
