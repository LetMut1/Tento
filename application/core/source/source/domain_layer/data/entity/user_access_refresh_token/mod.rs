mod field;
use {
    self::field::{
        ExpiresAt,
        ObfuscationValue,
        UpdatedAt,
    },
    super::{
        user::User_Id,
        user_access_token::UserAccessToken_ObfuscationValue,
        user_device::UserDevice_Id,
    },
};
pub struct UserAccessRefreshToken {
    user__id: User_Id,
    user_device__id: UserDevice_Id,
    user_access_token__obfuscation_value: UserAccessToken_ObfuscationValue,
    obfuscation_value: ObfuscationValue,
    expires_at: ExpiresAt,
    updated_at: UpdatedAt,
}
pub type UserAccessRefreshToken_ExpiresAt = ExpiresAt;
pub type UserAccessRefreshToken_ObfuscationValue = ObfuscationValue;
