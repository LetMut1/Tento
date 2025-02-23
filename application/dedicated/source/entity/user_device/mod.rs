mod field;
use {
    self::field::Id,
    super::user::User_Id,
    std::marker::PhantomData,
};
pub struct UserDevice {
    id: String,
    _id: PhantomData<Id>,
    user__id: i64,
    _user__id: PhantomData<User_Id>,
}
pub type UserDevice_Id = Id;
