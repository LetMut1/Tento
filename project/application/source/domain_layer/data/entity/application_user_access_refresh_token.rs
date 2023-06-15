use crate::domain_layer::functionality::service::getter::Getter;
use extern_crate::serde::Deserialize;
use extern_crate::serde::Serialize;
use std::borrow::Cow;
use super::application_user_access_token::ApplicationUserAccessToken_Id;
use super::application_user_device::ApplicationUserDevice_Id;
use super::application_user::ApplicationUser_Id;

pub use self::ObfuscationValue as ApplicationUserAccessRefreshToken_ObfuscationValue;
pub use self::ExpiresAt as ApplicationUserAccessRefreshToken_ExpiresAt;
pub use self::UpdatedAt as ApplicationUserAccessRefreshToken_UpdatedAt;

#[derive(Serialize, Deserialize)]
#[serde(crate = "extern_crate::serde")]
#[serde(transparent)]
pub struct ObfuscationValue(String);

impl ObfuscationValue {
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

#[derive(Clone, Copy, Serialize, Deserialize)]
#[serde(crate = "extern_crate::serde")]
#[serde(transparent)]
pub struct UpdatedAt(i64);

impl UpdatedAt {
    pub fn new(inner: i64) -> Self {
        return Self(inner);
    }

    pub fn get<'a>(&'a self) -> i64 {
        return self.0;
    }
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "extern_crate::serde")]
pub struct ApplicationUserAccessRefreshToken<'a> {
    application_user_id: ApplicationUser_Id,
    application_user_device_id: Cow<'a, ApplicationUserDevice_Id>,
    application_user_access_token_id: Cow<'a, ApplicationUserAccessToken_Id>,
    obfuscation_value: ObfuscationValue,
    expires_at: ExpiresAt,
    updated_at: UpdatedAt
}

impl<'a> ApplicationUserAccessRefreshToken<'a> {
    pub const QUANTITY_OF_MINUTES_FOR_EXPIRATION: i64 = 60 * 24 * 30 * 3;

    pub fn new(
        application_user_id: ApplicationUser_Id,
        application_user_device_id: Cow<'a, ApplicationUserDevice_Id>,
        application_user_access_token_id: Cow<'a, ApplicationUserAccessToken_Id>,
        obfuscation_value: ObfuscationValue,
        expires_at: ExpiresAt,
        updated_at: UpdatedAt
    ) -> Self {
        return Self {
            application_user_id,
            application_user_device_id,
            application_user_access_token_id,
            obfuscation_value,
            expires_at,
            updated_at
        };
    }

    pub fn get_application_user_id<'b>(&'b self) -> ApplicationUser_Id {
        return self.application_user_id;
    }

    pub fn get_application_user_device_id<'b>(&'b self) -> &'b ApplicationUserDevice_Id {
        return self.application_user_device_id.as_ref();
    }

    pub fn get_application_user_access_token_id<'b>(&'b self) -> &'b ApplicationUserAccessToken_Id {
        return self.application_user_access_token_id.as_ref();
    }

    pub fn get_obfuscation_value<'b>(&'b self) -> &'b ObfuscationValue {
        return &self.obfuscation_value;
    }

    pub fn get_expires_at<'b>(&'b self) -> ExpiresAt {
        return self.expires_at;
    }

    pub fn get_updated_at<'b>(&'b self) -> UpdatedAt {
        return self.updated_at;
    }

    pub fn set_application_user_access_token_id<'b >(
        &'b mut self,
        application_user_access_token_id: Cow<'a, ApplicationUserAccessToken_Id>
    ) -> &'b mut Self {
        self.application_user_access_token_id = application_user_access_token_id;

        return self;
    }

    pub fn set_obfuscation_value<'b>(&'b mut self, obfuscation_value: ObfuscationValue) -> &'b mut Self {
        self.obfuscation_value = obfuscation_value;

        return self;
    }

    pub fn set_expires_at<'b>(&'b mut self, expires_at: ExpiresAt) -> &'b mut Self {
        self.expires_at = expires_at;

        return self;
    }

    pub fn set_updated_at<'b>(&'b mut self, updated_at: UpdatedAt) -> &'b mut Self {
        self.updated_at = updated_at;

        return self;
    }
}

impl<'a> Getter<'a, ApplicationUser_Id> for ApplicationUserAccessRefreshToken<'_> {
    fn get(&'a self) -> ApplicationUser_Id {
        return self.get_application_user_id();
    }
}

impl<'a> Getter<'a, &'a ApplicationUserDevice_Id> for ApplicationUserAccessRefreshToken<'_> {
    fn get(&'a self) -> &'a ApplicationUserDevice_Id {
        return self.get_application_user_device_id();
    }
}

impl<'a> Getter<'a, &'a ApplicationUserAccessToken_Id> for ApplicationUserAccessRefreshToken<'_> {
    fn get(&'a self) -> &'a ApplicationUserAccessToken_Id {
        return self.get_application_user_access_token_id();
    }
}

impl<'a> Getter<'a, &'a ObfuscationValue> for ApplicationUserAccessRefreshToken<'_> {
    fn get(&'a self) -> &'a ObfuscationValue {
        return self.get_obfuscation_value();
    }
}

impl<'a> Getter<'a, ExpiresAt> for ApplicationUserAccessRefreshToken<'_> {
    fn get(&'a self) -> ExpiresAt {
        return self.get_expires_at();
    }
}

impl<'a> Getter<'a, UpdatedAt> for ApplicationUserAccessRefreshToken<'_> {
    fn get(&'a self) -> UpdatedAt {
        return self.get_updated_at();
    }
}

pub struct ApplicationUserAccessRefreshToken_1 {
    application_user_access_token_id: ApplicationUserAccessToken_Id,
    obfuscation_value: ObfuscationValue,
    expires_at: ExpiresAt,
    updated_at: UpdatedAt
}