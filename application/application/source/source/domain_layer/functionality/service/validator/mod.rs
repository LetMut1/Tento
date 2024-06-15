pub mod application_user__email;
pub mod application_user__id;
pub mod application_user__nickname;
pub mod application_user__password;
pub mod application_user_authorization_token__value;
pub mod application_user_device__id;
pub mod application_user_registration_token__value;
pub mod application_user_reset_password_token__value;
pub mod channel__description;
pub mod channel__id;
pub mod channel__linked_name;
pub mod channel__name;
pub mod channel__orientation;
use std::marker::PhantomData;
pub struct Validator<S> {
    _subject: PhantomData<S>,
}
