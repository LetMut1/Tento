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
};
pub struct UserAccessToken {
    id: Id,
    user__id: User_Id,
    user_device__id: UserDevice_Id,
    expires_at: ExpiresAt,
}
pub type UserAccessToken_Id = Id;
pub type UserAccessToken_ExpiresAt = ExpiresAt;
