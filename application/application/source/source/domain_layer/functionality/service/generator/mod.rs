pub mod user_access_refresh_token__expires_at;
pub mod user_access_refresh_token__obfuscation_value;
pub mod user_access_refresh_token__updated_at;
pub mod user_access_token__expires_at;
pub mod user_access_token__id;
pub mod user_authorization_token__can_be_resent_from;
pub mod user_authorization_token__expires_at;
pub mod user_authorization_token__value;
pub mod user_registration_token__can_be_resent_from;
pub mod user_registration_token__expires_at;
pub mod user_registration_token__value;
pub mod user_reset_password_token__can_be_resent_from;
pub mod user_reset_password_token__expires_at;
pub mod user_reset_password_token__value;
use std::marker::PhantomData;
pub struct Generator<S> {
    _subject: PhantomData<S>,
}
