mod field;
use super::user::User_Id;
use std::marker::PhantomData;
use self::field::Id;
pub struct UserDevice {
    pub id: String,
    _id: PhantomData<Id>,
    pub application_user__id: i64,
    _application_user__id: PhantomData<User_Id>,
}
impl UserDevice {
    pub fn new(id: String, application_user__id: i64) -> Self {
        return Self {
            id,
            _id: PhantomData,
            application_user__id,
            _application_user__id: PhantomData,
        };
    }
}
pub type UserDevice_Id = Id;