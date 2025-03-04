mod field;
use {
    self::field::{
        ExpiresAt,
        ObfuscationValue,
        UpdatedAt,
    },
    super::{
        user::User_Id,
        user_access_token::UserAccessToken_Id,
        user_device::UserDevice_Id,
    },
};
pub struct UserAccessRefreshToken {
    user__id: User_Id,
    user_device__id: UserDevice_Id,
    user_access_token__id: UserAccessToken_Id,
    obfuscation_value: ObfuscationValue,
    expires_at: ExpiresAt,
    updated_at: UpdatedAt,
}
pub type UserAccessRefreshToken_ExpiresAt = ExpiresAt;
pub type UserAccessRefreshToken_ObfuscationValue = ObfuscationValue;
