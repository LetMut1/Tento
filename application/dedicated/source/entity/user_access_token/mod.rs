mod field;
use {
    self::field::{
        ExpiresAt,
        Id,
    },
    super::{
        user::User_Id,
        user_device::UserDevice_Id,
    },
    std::marker::PhantomData,
};
pub struct UserAccessToken {
    id: String,
    _id: PhantomData<Id>,
    user__id: i64,
    _user__id: PhantomData<User_Id>,
    user_device__id: String,
    _user_device__id: PhantomData<UserDevice_Id>,
    expires_at: i64,
    _expires_at: PhantomData<ExpiresAt>,
}
pub type UserAccessToken_Id = Id;
pub type UserAccessToken_ExpiresAt = ExpiresAt;
