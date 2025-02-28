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
    std::marker::PhantomData,
};
pub struct UserAccessRefreshToken {
    user__id: PhantomData<(i64, User_Id)>,
    user_device__id: PhantomData<(String, UserDevice_Id)>,
    user_access_token__id: PhantomData<(String, UserAccessToken_Id)>,
    obfuscation_value: PhantomData<(String, ObfuscationValue)>,
    expires_at: PhantomData<(i64, ExpiresAt)>,
    updated_at: PhantomData<(i64, UpdatedAt)>,
}
pub type UserAccessRefreshToken_ExpiresAt = ExpiresAt;
pub type UserAccessRefreshToken_ObfuscationValue = ObfuscationValue;
pub type UserAccessRefreshToken_UpdatedAt = UpdatedAt;
