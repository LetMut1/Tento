pub mod user__password;
pub mod user_access_refresh_token;
pub mod user_access_token;
use std::marker::PhantomData;
pub struct Encoder<S> {
    _subject: PhantomData<S>,
}
