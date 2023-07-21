use super::application_user::ApplicationUser_Id;
use extern_crate::serde::Deserialize;
use extern_crate::serde::Serialize;

pub use self::Id as ApplicationUserDevice_Id;

#[derive(Clone, Serialize, Deserialize)]
#[serde(crate = "extern_crate::serde")]
#[serde(transparent)]
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
    pub id: Id,
    pub application_user_id: ApplicationUser_Id,
}