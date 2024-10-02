mod field;
use self::field::{
    ExpiresAt,
    Id,
};
use super::{
    user::User_Id,
    user_device::UserDevice_Id,
};
use serde::{
    Deserialize,
    Serialize,
};
use std::{
    borrow::Cow,
    marker::PhantomData,
};
#[derive(Serialize, Deserialize)]
pub struct UserAccessToken<'a> {
    pub id: String,
    _id: PhantomData<Id>,

    pub application_user__id: i64,
    _application_user__id: PhantomData<User_Id>,

    pub application_user_device__id: Cow<'a, str>,
    _application_user_device__id: PhantomData<UserDevice_Id>,

    pub expires_at: i64,
    _expires_at: PhantomData<ExpiresAt>,
}
impl<'a> UserAccessToken<'a> {
    pub fn new(id: String, application_user__id: i64, application_user_device__id: Cow<'a, str>, expires_at: i64) -> Self {
        return Self {
            id,
            _id: PhantomData,
            application_user__id,
            _application_user_device__id: PhantomData,
            _application_user__id: PhantomData,
            application_user_device__id,
            expires_at,
            _expires_at: PhantomData,
        };
    }
}
pub type UserAccessToken_Id = Id;
pub type UserAccessToken_ExpiresAt = ExpiresAt;