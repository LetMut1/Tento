mod field;
use {
    self::field::{
        ExpiresAt,
        ObfuscationValue,
    },
    super::{
        user::User_Id,
        user_device::UserDevice_Id,
    },
};
pub struct UserAccessToken {
    user__id: User_Id,
    user_device__id: UserDevice_Id,
    obfuscation_value: ObfuscationValue,
    expires_at: ExpiresAt,
}
pub type UserAccessToken_ObfuscationValue = ObfuscationValue;
pub type UserAccessToken_ExpiresAt = ExpiresAt;
