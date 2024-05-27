use super::application_user::ApplicationUser_Id;
use super::application_user_device::ApplicationUserDevice_Id;
use serde::Deserialize;
use serde::Serialize;
use std::borrow::Cow;
use std::marker::PhantomData;

pub use self::ExpiresAt as ApplicationUserAccessToken_ExpiresAt;
pub use self::Id as ApplicationUserAccessToken_Id;

#[derive(Serialize, Deserialize)]
pub struct ApplicationUserAccessToken<'a> {
    pub id: Id,
    pub application_user_id: i64,
    _application_user_id: PhantomData<ApplicationUser_Id>,

    pub application_user_device_id: Cow<'a, ApplicationUserDevice_Id>,
    pub expires_at: ExpiresAt,
}

impl<'a> ApplicationUserAccessToken<'a> {
    pub fn new(
        id: Id,
        application_user_id: i64,
        application_user_device_id: Cow<'a, ApplicationUserDevice_Id>,
        expires_at: ExpiresAt,
    ) -> Self {
        return Self {
            id,
            application_user_id,
            _application_user_id: PhantomData,
            application_user_device_id,
            expires_at,
        };
    }
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Id(pub String);

#[derive(Clone, Copy, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ExpiresAt(pub i64);

impl ExpiresAt {
    pub const QUANTITY_OF_MINUTES_FOR_EXPIRATION: i64 = 30;
}