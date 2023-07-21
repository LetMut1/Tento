use super::application_user::ApplicationUser_Id;
use super::application_user_device::ApplicationUserDevice_Id;
use extern_crate::serde::Deserialize;
use extern_crate::serde::Serialize;
use std::borrow::Cow;

pub use self::ExpiresAt as ApplicationUserAccessToken_ExpiresAt;
pub use self::Id as ApplicationUserAccessToken_Id;

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

#[derive(Clone, Copy, Serialize, Deserialize)]
#[serde(crate = "extern_crate::serde")]
#[serde(transparent)]
pub struct ExpiresAt(i64);

impl ExpiresAt {
    pub fn new(inner: i64) -> Self {
        return Self(inner);
    }

    pub fn get<'a>(&'a self) -> i64 {
        return self.0;
    }
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "extern_crate::serde")]
pub struct ApplicationUserAccessToken<'a> {
    pub id: Id,
    pub application_user_id: ApplicationUser_Id,
    pub application_user_device_id: Cow<'a, ApplicationUserDevice_Id>,
    pub expires_at: ExpiresAt,
}

impl<'a> ApplicationUserAccessToken<'a> {
    pub const QUANTITY_OF_MINUTES_FOR_EXPIRATION: i64 = 30;
}
