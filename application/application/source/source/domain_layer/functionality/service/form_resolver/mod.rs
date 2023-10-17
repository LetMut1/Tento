pub mod application_user_access_refresh_token;
pub mod application_user_access_token;
pub mod channel__access_modifier;
pub mod channel__visability_modifier;

use std::marker::PhantomData;

pub struct FormResolver<S> {
    _subject: PhantomData<S>,
}
