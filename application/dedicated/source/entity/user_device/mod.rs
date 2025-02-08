mod field;
use {
    self::field::Id,
    super::user::User_Id,
    std::marker::PhantomData,
};
pub struct UserDevice {
    pub id: String,
    _id: PhantomData<Id>,
    pub user__id: i64,
    _user__id: PhantomData<User_Id>,
}
impl UserDevice {
    pub fn new(id: String, user__id: i64) -> Self {
        return Self {
            id,
            _id: PhantomData,
            user__id,
            _user__id: PhantomData,
        };
    }
}
pub type UserDevice_Id = Id;
