use extern_crate::serde::Deserialize;
use extern_crate::serde::Serialize;
use std::borrow::Cow;
use std::marker::PhantomData;
use super::application_user_access_token::ApplicationUserAccessToken_Id;
use super::application_user_device::ApplicationUserDevice_Id;
use super::application_user::ApplicationUser_Id;

pub use self::ObfuscationValue as ApplicationUserAccessRefreshToken_ObfuscationValue;
pub use self::ExpiresAt as ApplicationUserAccessRefreshToken_ExpiresAt;
pub use self::UpdatedAt as ApplicationUserAccessRefreshToken_UpdatedAt;

#[derive(Serialize, Deserialize)]
#[serde(crate = "extern_crate::serde")]
pub struct ApplicationUserAccessRefreshToken<'a> {
    application_user_id: i64,
    _application_user_id: PhantomData<ApplicationUser_Id>,

    application_user_device_id: Cow<'a, str>,
    _application_user_device_id: PhantomData<ApplicationUserDevice_Id>,

    application_user_access_token_id: Cow<'a, str>,
    _application_user_access_token_id: PhantomData<ApplicationUserAccessToken_Id>,

    obfuscation_value: String,
    _obfuscation_value: PhantomData<ObfuscationValue>,

    expires_at: i64,
    _expires_at: PhantomData<ExpiresAt>,

    updated_at: i64,
    _updated_at: PhantomData<UpdatedAt>,
}

impl<'a> ApplicationUserAccessRefreshToken<'a> {
    pub const QUANTITY_OF_MINUTES_FOR_EXPIRATION: i64 = 60 * 24 * 30 * 3;

    pub fn new(
        application_user_id: i64,
        application_user_device_id: Cow<'a, str>,
        application_user_access_token_id: Cow<'a, str>,
        obfuscation_value: String,
        expires_at: i64,
        updated_at: i64
    ) -> Self {
        return Self {
            application_user_id,
            _application_user_id: PhantomData,
            application_user_device_id,
            _application_user_device_id: PhantomData,
            application_user_access_token_id,
            _application_user_access_token_id: PhantomData,
            obfuscation_value,
            _obfuscation_value: PhantomData,
            expires_at,
            _expires_at: PhantomData,
            updated_at,
            _updated_at: PhantomData
        };
    }

    pub fn get_application_user_id<'b>(&'b self) -> i64 {
        return self.application_user_id;
    }

    pub fn get_application_user_device_id<'b>(&'b self) -> &'b str {
        return self.application_user_device_id.as_ref();
    }

    pub fn get_application_user_access_token_id<'b>(&'b self) -> &'b str {
        return self.application_user_access_token_id.as_ref();
    }

    pub fn get_obfuscation_value<'b>(&'b self) -> &'b str {
        return self.obfuscation_value.as_str();
    }

    pub fn get_expires_at<'b>(&'b self) -> i64 {
        return self.expires_at;
    }

    pub fn get_updated_at<'b>(&'b self) -> i64 {
        return self.updated_at;
    }

    pub fn set_application_user_access_token_id<'b >(&'b mut self, application_user_access_token_id: Cow<'a, str>) -> &'b mut Self {
        self.application_user_access_token_id = application_user_access_token_id;

        return self;
    }

    pub fn set_obfuscation_value<'b>(&'b mut self, obfuscation_value: String) -> &'b mut Self {
        self.obfuscation_value = obfuscation_value;

        return self;
    }

    pub fn set_expires_at<'b>(&'b mut self, expires_at: i64) -> &'b mut Self {
        self.expires_at = expires_at;

        return self;
    }

    pub fn set_updated_at<'b>(&'b mut self, updated_at: i64) -> &'b mut Self {
        self.updated_at = updated_at;

        return self;
    }
}

pub struct ObfuscationValue;

pub struct ExpiresAt;

pub struct UpdatedAt;