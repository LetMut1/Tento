pub use self::Id as ApplicationUserDevice_Id;
use super::application_user::ApplicationUser_Id;
use std::marker::PhantomData;
pub struct ApplicationUserDevice {
    pub id: String,
    _id: PhantomData<Id>,

    pub application_user__id: i64,
    _application_user__id: PhantomData<ApplicationUser_Id>,
}
impl ApplicationUserDevice {
    pub fn new(id: String, application_user__id: i64) -> Self {
        return Self {
            id,
            _id: PhantomData,
            application_user__id,
            _application_user__id: PhantomData,
        };
    }
}
pub struct Id;
