pub use self::{
    ExpiresAt as ApplicationUserAccessRefreshToken_ExpiresAt,
    ObfuscationValue as ApplicationUserAccessRefreshToken_ObfuscationValue,
    UpdatedAt as ApplicationUserAccessRefreshToken_UpdatedAt,
};
use super::{
    application_user::ApplicationUser_Id,
    application_user_access_token::ApplicationUserAccessToken_Id,
    application_user_device::ApplicationUserDevice_Id,
};
use serde::{
    Deserialize,
    Serialize,
};
use std::{
    borrow::Cow,
    marker::PhantomData,
};
#[derive(Serialize, Deserialize)]
pub struct ApplicationUserAccessRefreshToken<'a> {
    pub application_user__id: i64,
    _application_user__id: PhantomData<ApplicationUser_Id>,

    pub application_user_device__id: Cow<'a, str>,
    _application_user_device__id: PhantomData<ApplicationUserDevice_Id>,

    pub application_user_access_token__id: Cow<'a, str>,
    _application_user_access_token__id: PhantomData<ApplicationUserAccessToken_Id>,

    pub obfuscation_value: String,
    _obfuscation_value: PhantomData<ObfuscationValue>,

    pub expires_at: i64,
    _expires_at: PhantomData<ExpiresAt>,

    pub updated_at: i64,
    _updated_at: PhantomData<UpdatedAt>,
}
impl<'a> ApplicationUserAccessRefreshToken<'a> {
    pub fn new(
        application_user__id: i64,
        application_user_device__id: Cow<'a, str>,
        application_user_access_token__id: Cow<'a, str>,
        obfuscation_value: String,
        expires_at: i64,
        updated_at: i64,
    ) -> Self {
        return Self {
            application_user__id,
            _application_user__id: PhantomData,
            application_user_device__id,
            _application_user_device__id: PhantomData,
            application_user_access_token__id,
            _application_user_access_token__id: PhantomData,
            obfuscation_value,
            _obfuscation_value: PhantomData,
            expires_at,
            _expires_at: PhantomData,
            updated_at,
            _updated_at: PhantomData,
        };
    }
}
pub struct ObfuscationValue;
pub struct ExpiresAt;
impl ExpiresAt {
    pub const QUANTITY_OF_MINUTES_FOR_EXPIRATION: i64 = 60 * 24 * 30 * 3;
}
pub struct UpdatedAt;
