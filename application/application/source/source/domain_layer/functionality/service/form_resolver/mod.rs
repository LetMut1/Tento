pub mod application_user_access_refresh_token;
pub mod application_user_access_token;
use std::marker::PhantomData;
pub struct FormResolver<S> {
    _subject: PhantomData<S>,
}
