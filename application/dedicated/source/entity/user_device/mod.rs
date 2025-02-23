mod field;
use {
    self::field::Id,
    super::user::User_Id,
    std::marker::PhantomData,
};
pub struct UserDevice {
    id: PhantomData<(String, Id)>,
    user__id: PhantomData<(i64, User_Id)>,
}
pub type UserDevice_Id = Id;
