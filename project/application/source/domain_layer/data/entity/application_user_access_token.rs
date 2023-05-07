use extern_crate::serde::Deserialize;
use extern_crate::serde::Serialize;
use std::borrow::Cow;
use super::application_user_device::ApplicationUserDevice_Id;
use super::application_user::ApplicationUser_Id;

pub use self::Id as ApplicationUserAccessToken_Id;
pub use self::ExpiresAt as ApplicationUserAccessToken_ExpiresAt;

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

#[derive(Clone, Copy, Serialize, Deserialize)]
#[serde(crate = "extern_crate::serde")]
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
    id: Id,
    application_user_id: ApplicationUser_Id,
    application_user_device_id: Cow<'a, ApplicationUserDevice_Id>,
    expires_at: ExpiresAt
}

impl<'a> ApplicationUserAccessToken<'a> {
    pub const QUANTITY_OF_MINUTES_FOR_EXPIRATION: i64 = 30;

    pub fn new(
        id: Id,
        application_user_id: ApplicationUser_Id,
        application_user_device_id: Cow<'a, ApplicationUserDevice_Id>,
        expires_at: ExpiresAt
    ) -> Self {
        return Self {
            id,
            application_user_id,
            application_user_device_id,
            expires_at
        };
    }

    pub fn get_id<'b>(&'b self) -> &'b Id {
        return &self.id;
    }

    pub fn get_application_user_id<'b>(&'b self) -> ApplicationUser_Id {
        return self.application_user_id;
    }

    pub fn get_application_user_device_id<'b>(&'b self) -> &'b ApplicationUserDevice_Id {
        return self.application_user_device_id.as_ref();
    }

    pub fn get_expires_at<'b>(&'b self) -> ExpiresAt {
        return self.expires_at;
    }
}