pub mod derivative;
pub use self::{
    CanBeResentFrom as ApplicationUserAuthorizationToken_CanBeResentFrom,
    ExpiresAt as ApplicationUserAuthorizationToken_ExpiresAt,
    Value as ApplicationUserAuthorizationToken_Value,
    WrongEnterTriesQuantity as ApplicationUserAuthorizationToken_WrongEnterTriesQuantity,
};
use super::{
    application_user::ApplicationUser_Id,
    application_user_device::ApplicationUserDevice_Id,
};
use std::{
    borrow::Cow,
    marker::PhantomData,
};
pub struct ApplicationUserAuthorizationToken<'a> {
    pub application_user__id: i64,
    _application_user__id: PhantomData<ApplicationUser_Id>,

    pub application_user_device__id: Cow<'a, str>,
    _application_user_device__id: PhantomData<ApplicationUserDevice_Id>,

    pub value: String,
    _value: PhantomData<Value>,

    pub wrong_enter_tries_quantity: i16,
    _wrong_enter_tries_quantity: PhantomData<WrongEnterTriesQuantity>,

    pub expires_at: i64,
    _expires_at: PhantomData<ExpiresAt>,

    pub can_be_resent_from: i64,
    _can_be_resent_from: PhantomData<CanBeResentFrom>,
}
impl<'a> ApplicationUserAuthorizationToken<'a> {
    pub fn new(
        application_user__id: i64,
        application_user_device__id: Cow<'a, str>,
        value: String,
        wrong_enter_tries_quantity: i16,
        expires_at: i64,
        can_be_resent_from: i64,
    ) -> Self {
        return Self {
            application_user__id,
            _application_user__id: PhantomData,
            application_user_device__id,
            _application_user_device__id: PhantomData,
            value,
            _value: PhantomData,
            wrong_enter_tries_quantity,
            _wrong_enter_tries_quantity: PhantomData,
            expires_at,
            _expires_at: PhantomData,
            can_be_resent_from,
            _can_be_resent_from: PhantomData,
        };
    }
}
pub struct Value;
impl Value {
    pub const REGULAR_EXPRESSION: &'static str = r#"^[0-9]{6}$"#;
}
pub struct WrongEnterTriesQuantity;
impl WrongEnterTriesQuantity {
    pub const LIMIT: i16 = 5;
}
pub struct ExpiresAt;
impl ExpiresAt {
    pub const QUANTITY_OF_MINUTES_FOR_EXPIRATION: i64 = 10;
}
pub struct CanBeResentFrom;
impl CanBeResentFrom {
    pub const QUANTITY_OF_MINUTES_BEFORE_RESENDING: i64 = 1;
}
