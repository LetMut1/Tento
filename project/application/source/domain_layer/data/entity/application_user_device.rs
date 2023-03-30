use std::marker::PhantomData;
use super::application_user::Id as ApplicationUserId;

pub struct ApplicationUserDevice {
    id: String,
    _id: PhantomData<Id>,

    application_user_id: i64,
    _application_user_id: PhantomData<ApplicationUserId>
}

impl ApplicationUserDevice {
    pub fn new(
        id: String,
        application_user_id: i64
    ) -> Self {
        return Self {
            id,
            _id: PhantomData,
            application_user_id,
            _application_user_id: PhantomData
        };
    }

    pub fn get_id<'a>(&'a self) -> &'a str {
        return self.id.as_str();
    }
}

pub struct Id;