pub mod application_user__email___validator;
pub mod application_user__id___validator;
pub mod application_user__nickname___validator;
pub mod application_user__password___validator;
pub mod application_user_authorization_token__value___validator;
pub mod application_user_device__id___validator;
pub mod application_user_registration_token__value___validator;
pub mod application_user_reset_password_token__value___validator;
pub mod channel__description___validator;
pub mod channel__id___validator;
pub mod channel__linked_name___validator;
pub mod channel__name___validator;
pub mod channel__orientation___validator;

use std::marker::PhantomData;

pub struct Validator<S> {
    _subject: PhantomData<S>,
}
