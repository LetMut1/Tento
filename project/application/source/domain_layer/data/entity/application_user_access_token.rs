use extern_crate::serde::Deserialize;
use extern_crate::serde::Serialize;
use std::borrow::Cow;
use std::marker::PhantomData;
use super::application_user_device::ApplicationUserDevice_Id;
use super::application_user::ApplicationUser_Id;

#[derive(Serialize, Deserialize)]
#[serde(crate = "extern_crate::serde")]
pub struct ApplicationUserAccessToken<'a> {
    id: String,
    _id: PhantomData<Id>,

    application_user_id: i64,
    _application_user_id: PhantomData<ApplicationUser_Id>,

    application_user_device_id: Cow<'a, str>,
    _application_user_device_id: PhantomData<ApplicationUserDevice_Id>,

    expires_at: i64,
    _expires_at: PhantomData<ExpiresAt>
}

impl<'a> ApplicationUserAccessToken<'a> {
    pub const QUANTITY_OF_MINUTES_FOR_EXPIRATION: i64 = 30;

    pub fn new(
        id: String,
        application_user_id: i64,
        application_user_device_id: Cow<'a, str>,
        expires_at: i64
    ) -> Self {
        return Self {
            id,
            _id: PhantomData,
            application_user_id,
            _application_user_id: PhantomData,
            application_user_device_id,
            _application_user_device_id: PhantomData,
            expires_at,
            _expires_at: PhantomData
        };
    }

    pub fn get_id<'b>(&'b self) -> &'b str {
        return self.id.as_str();
    }

    pub fn get_application_user_id<'b>(&'b self) -> i64 {
        return self.application_user_id;
    }

    pub fn get_application_user_device_id<'b>(&'b self) -> &'b str {
        return self.application_user_device_id.as_ref();
    }

    pub fn get_expires_at<'b>(&'b self) -> i64 {
        return self.expires_at;
    }
}

pub struct Id;

pub struct ExpiresAt;