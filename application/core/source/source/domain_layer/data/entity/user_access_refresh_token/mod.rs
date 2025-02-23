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
    user__id: i64,
    _user__id: PhantomData<User_Id>,
    user_device__id: String,
    _user_device__id: PhantomData<UserDevice_Id>,
    user_access_token__id: String,
    _user_access_token__id: PhantomData<UserAccessToken_Id>,
    obfuscation_value: String,
    _obfuscation_value: PhantomData<ObfuscationValue>,
    expires_at: i64,
    _expires_at: PhantomData<ExpiresAt>,
    updated_at: i64,
    _updated_at: PhantomData<UpdatedAt>,
}
pub type UserAccessRefreshToken_ExpiresAt = ExpiresAt;
pub type UserAccessRefreshToken_ObfuscationValue = ObfuscationValue;
pub type UserAccessRefreshToken_UpdatedAt = UpdatedAt;
