mod user_access_refresh_token__expires_at;
mod user_access_refresh_token__obfuscation_value;
mod user_access_token__expires_at;
mod user_access_token__id;
mod user_authorization_token__can_be_resent_from;
mod user_authorization_token__expires_at;
mod user_authorization_token__value;
mod user_registration_token__can_be_resent_from;
mod user_registration_token__expires_at;
mod user_registration_token__value;
mod user_reset_password_token__can_be_resent_from;
mod user_reset_password_token__expires_at;
mod user_reset_password_token__value;
use std::marker::PhantomData;
pub struct Generator<S> {
    _subject: PhantomData<S>,
}
