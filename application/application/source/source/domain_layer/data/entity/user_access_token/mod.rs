mod field;
use self::field::{
    ExpiresAt,
    Id,
};
use super::{
    user::User_Id,
    user_device::UserDevice_Id,
};
use std::marker::PhantomData;
#[cfg_attr(feature = "serde_for_manual_test", derive(serde::Serialize, serde::Deserialize))]
#[derive(bitcode::Encode, bitcode::Decode)]
pub struct UserAccessToken<'a> {
    pub id: String,
    _id: PhantomData<Id>,
    pub user__id: i64,
    _user__id: PhantomData<User_Id>,
    pub user_device__id: &'a str,
    _user_device__id: PhantomData<UserDevice_Id>,
    pub expires_at: i64,
    _expires_at: PhantomData<ExpiresAt>,
}
impl<'a> UserAccessToken<'a> {
    pub fn new(id: String, user__id: i64, user_device__id: &'a str, expires_at: i64) -> Self {
        return Self {
            id,
            _id: PhantomData,
            user__id,
            _user_device__id: PhantomData,
            _user__id: PhantomData,
            user_device__id,
            expires_at,
            _expires_at: PhantomData,
        };
    }
}
pub type UserAccessToken_Id = Id;
pub type UserAccessToken_ExpiresAt = ExpiresAt;