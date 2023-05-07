use extern_crate::serde::Deserialize;
use extern_crate::serde::Serialize;
use std::marker::PhantomData;
use super::application_user::ApplicationUser_Id;

pub use self::Id as ApplicationUserDevice_Id;

#[derive(Clone, Serialize, Deserialize)]
#[serde(crate = "extern_crate::serde")]
pub struct Id(String);

impl Id {
    pub fn new(inner: String) -> Self {
        return Self(inner);
    }

    pub fn get<'a>(&'a self) -> &'a str {
        return self.0.as_str();
    }
}

pub struct ApplicationUserDevice {
    id: String,
    _id: PhantomData<Id>,

    application_user_id: i64,
    _application_user_id: PhantomData<ApplicationUser_Id>
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