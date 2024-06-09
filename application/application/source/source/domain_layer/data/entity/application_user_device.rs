use super::application_user::ApplicationUser_Id;
use std::marker::PhantomData;

pub use self::Id as ApplicationUserDevice_Id;

pub struct ApplicationUserDevice {
    pub id: String,
    _id: PhantomData<Id>,

    pub application_user_id: i64,
    _application_user_id: PhantomData<ApplicationUser_Id>,
}

impl ApplicationUserDevice {
    pub fn new(
        id: String,
        application_user_id: i64,
    ) -> Self {
        return Self {
            id,
            _id: PhantomData,
            application_user_id,
            _application_user_id: PhantomData,
        };
    }
}

pub struct Id;
