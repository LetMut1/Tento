pub mod user__email;
pub mod user__id;
pub mod user__nickname;
pub mod user__password;
pub mod user_authorization_token__value;
pub mod user_device__id;
pub mod user_registration_token__value;
pub mod user_reset_password_token__value;
pub mod channel__description;
pub mod channel__id;
pub mod channel__linked_name;
pub mod channel__name;
pub mod channel__orientation;
use std::marker::PhantomData;
pub struct Validator<S> {
    _subject: PhantomData<S>,
}
