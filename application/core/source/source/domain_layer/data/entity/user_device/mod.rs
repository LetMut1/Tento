mod field;
use {
    self::field::Id,
    super::user::User_Id,
};
pub struct UserDevice {
    id: Id,
    user__id: User_Id,
}
pub type UserDevice_Id = Id;
