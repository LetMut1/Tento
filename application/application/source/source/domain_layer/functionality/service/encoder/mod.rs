pub mod application_user__password;
use std::marker::PhantomData;
pub struct Encoder<S> {
    _subject: PhantomData<S>,
}
