mod user__password;
mod user_access_refresh_token;
mod user_access_token;
use std::marker::PhantomData;
pub struct Encoder<S> {
    _subject: PhantomData<S>,
}
