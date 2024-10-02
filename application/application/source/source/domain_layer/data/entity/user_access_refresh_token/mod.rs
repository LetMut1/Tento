mod field;
use self::field::{
    ExpiresAt,
    ObfuscationValue,
    UpdatedAt,
};
use super::user::User_Id;
use super::user_device::UserDevice_Id;
use super::user_access_token::UserAccessToken_Id;
use serde::{
    Deserialize,
    Serialize,
};
use std::{
    borrow::Cow,
    marker::PhantomData,
};
#[derive(Serialize, Deserialize)]
pub struct UserAccessRefreshToken<'a> {
    pub user__id: i64,
    _user__id: PhantomData<User_Id>,
    pub user_device__id: Cow<'a, str>,
    _user_device__id: PhantomData<UserDevice_Id>,
    pub application_user_access_token__id: Cow<'a, str>,
    _application_user_access_token__id: PhantomData<UserAccessToken_Id>,
    pub obfuscation_value: String,
    _obfuscation_value: PhantomData<ObfuscationValue>,
    pub expires_at: i64,
    _expires_at: PhantomData<ExpiresAt>,
    pub updated_at: i64,
    _updated_at: PhantomData<UpdatedAt>,
}
impl<'a> UserAccessRefreshToken<'a> {
    pub fn new(
        user__id: i64,
        user_device__id: Cow<'a, str>,
        application_user_access_token__id: Cow<'a, str>,
        obfuscation_value: String,
        expires_at: i64,
        updated_at: i64,
    ) -> Self {
        return Self {
            user__id,
            _user__id: PhantomData,
            user_device__id,
            _user_device__id: PhantomData,
            application_user_access_token__id,
            _application_user_access_token__id: PhantomData,
            obfuscation_value,
            _obfuscation_value: PhantomData,
            expires_at,
            _expires_at: PhantomData,
            updated_at,
            _updated_at: PhantomData,
        };
    }
}
pub type UserAccessRefreshToken_ExpiresAt = ExpiresAt;
pub type UserAccessRefreshToken_ObfuscationValue = ObfuscationValue;
pub type UserAccessRefreshToken_UpdatedAt = UpdatedAt;