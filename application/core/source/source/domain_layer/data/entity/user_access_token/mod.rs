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
    id: PhantomData<(String, Id)>,
    user__id: PhantomData<(i64, User_Id)>,
    user_device__id: PhantomData<(String, UserDevice_Id)>,
    expires_at: PhantomData<(i64, ExpiresAt)>,
}
pub type UserAccessToken_Id = Id;
pub type UserAccessToken_ExpiresAt = ExpiresAt;
