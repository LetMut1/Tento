use super::application_user::ApplicationUser_Id;
use super::application_user_device::ApplicationUserDevice_Id;
use serde::Deserialize;
use serde::Serialize;
use std::borrow::Cow;

pub use self::ExpiresAt as ApplicationUserAccessToken_ExpiresAt;
pub use self::Id as ApplicationUserAccessToken_Id;

#[derive(Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Id(pub String);

#[derive(Clone, Copy, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ExpiresAt(pub i64);

impl ExpiresAt {
    pub const QUANTITY_OF_MINUTES_FOR_EXPIRATION: i64 = 30;
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationUserAccessToken<'a> {
    pub id: Id,
    pub application_user_id: ApplicationUser_Id,
    pub application_user_device_id: Cow<'a, ApplicationUserDevice_Id>,
    pub expires_at: ExpiresAt,
}
