mod user_authorization_token;
mod user_registration_token;
mod user_reset_password_token;
use std::marker::PhantomData;
pub struct EmailSender<S> {
    _subject: PhantomData<S>,
}
