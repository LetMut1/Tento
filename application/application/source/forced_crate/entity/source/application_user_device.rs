use super::application_user::ApplicationUser_Id;
use serde::Deserialize;
use serde::Serialize;
use std::marker::PhantomData;

pub use self::Id as ApplicationUserDevice_Id;

pub struct ApplicationUserDevice {
    pub id: Id,
    pub application_user_id: i64,
    _application_user_id: PhantomData<ApplicationUser_Id>,
}

impl ApplicationUserDevice {
    pub fn new(
        id: Id,
        application_user_id: i64,
    ) -> Self {
        return Self {
            id,
            application_user_id,
            _application_user_id: PhantomData,
        };
    }
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Id(pub String);
