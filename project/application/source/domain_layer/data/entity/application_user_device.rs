use extern_crate::serde::Deserialize;
use extern_crate::serde::Serialize;
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
    id: Id,
    application_user_id: ApplicationUser_Id
}

impl ApplicationUserDevice {
    pub fn new(
        id: Id,
        application_user_id: ApplicationUser_Id
    ) -> Self {
        return Self {
            id,
            application_user_id
        };
    }

    pub fn get_id<'a>(&'a self) -> &'a Id {
        return &self.id;
    }
}