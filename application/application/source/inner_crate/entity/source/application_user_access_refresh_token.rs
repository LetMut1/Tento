use super::application_user::ApplicationUser_Id;
use super::application_user_access_token::ApplicationUserAccessToken_Id;
use super::application_user_device::ApplicationUserDevice_Id;
use serde::Deserialize;
use serde::Serialize;
use std::borrow::Cow;

pub use self::ExpiresAt as ApplicationUserAccessRefreshToken_ExpiresAt;
pub use self::ObfuscationValue as ApplicationUserAccessRefreshToken_ObfuscationValue;
pub use self::UpdatedAt as ApplicationUserAccessRefreshToken_UpdatedAt;

#[derive(Serialize, Deserialize)]
#[serde(transparent)]
pub struct ObfuscationValue(pub String);

#[derive(Clone, Copy, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ExpiresAt(pub i64);

impl ExpiresAt {
    pub const QUANTITY_OF_MINUTES_FOR_EXPIRATION: i64 = 60 * 24 * 30 * 3;
}

#[derive(Clone, Copy, Serialize, Deserialize)]
#[serde(transparent)]
pub struct UpdatedAt(pub i64);

#[derive(Serialize, Deserialize)]
pub struct ApplicationUserAccessRefreshToken<'a> {
    pub application_user_id: ApplicationUser_Id,
    pub application_user_device_id: Cow<'a, ApplicationUserDevice_Id>,
    pub application_user_access_token_id: Cow<'a, ApplicationUserAccessToken_Id>,
    pub obfuscation_value: ObfuscationValue,
    pub expires_at: ExpiresAt,
    pub updated_at: UpdatedAt,
}

pub struct ApplicationUserAccessRefreshToken1 {
    pub application_user_access_token_id: ApplicationUserAccessToken_Id,
    pub obfuscation_value: ObfuscationValue,
    pub expires_at: ExpiresAt,
    pub updated_at: UpdatedAt,
}
