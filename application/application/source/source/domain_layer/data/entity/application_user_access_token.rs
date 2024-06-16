pub use self::{
    ExpiresAt as ApplicationUserAccessToken_ExpiresAt,
    Id as ApplicationUserAccessToken_Id,
};
use super::{
    application_user::ApplicationUser_Id,
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
pub struct ApplicationUserAccessToken<'a> {
    pub id: String,
    _id: PhantomData<Id>,

    pub application_user__id: i64,
    _application_user__id: PhantomData<ApplicationUser_Id>,

    pub application_user_device__id: Cow<'a, str>,
    _application_user_device__id: PhantomData<ApplicationUserDevice_Id>,

    pub expires_at: i64,
    _expires_at: PhantomData<ExpiresAt>,
}
impl<'a> ApplicationUserAccessToken<'a> {
    pub fn new(id: String, application_user__id: i64, application_user_device__id: Cow<'a, str>, expires_at: i64) -> Self {
        return Self {
            id,
            _id: PhantomData,
            application_user__id,
            _application_user_device__id: PhantomData,
            _application_user__id: PhantomData,
            application_user_device__id,
            expires_at,
            _expires_at: PhantomData,
        };
    }
}
pub struct Id;
pub struct ExpiresAt;
impl ExpiresAt {
    pub const QUANTITY_OF_MINUTES_FOR_EXPIRATION: i64 = 30;
}
